# Architecture Typed - Migration vers Type Safety & Performance

## Vue d'ensemble

Cette architecture en 6 étapes transforme progressivement le système d'évaluation actuel (basé sur `Bound<PyAny>`) vers un système fortement typé utilisant des types Rust concrets et des marqueurs de types à la compilation.

**Objectif final** : Zero-cost abstractions avec maximum de type safety, tout en gardant l'API Python simple.

---

## 📊 Tableau récapitulatif

| Étape | LOC | Complexité | Perf | Safety | Flexibility | Indépendante |
|-------|-----|------------|------|--------|-------------|--------------|
| 1. TypedValue | ~150 | ⭐ Faible | +5% | +++ | +++ | ✅ Oui |
| 2. Typed eval_*_op | ~200 | ⭐⭐ Moyenne | +10% | ++++ | ++ | ✅ Oui |
| 3. eval_any migration | ~100 | ⭐⭐ Moyenne | +15% | ++++ | + | ❌ Dépend de 1+2 |
| 4. Phantom Types | ~150 | ⭐⭐⭐ Élevée | +5% | +++++ | +++ | ✅ Oui |
| 5. Typed NameSpaces | ~150 | ⭐⭐⭐ Élevée | +10% | +++++ | ++++ | ❌ Dépend de 4 |
| 6. Full optimization | ~200 | ⭐⭐⭐⭐ Très élevée | +20% | +++++ | +++++ | ❌ Dépend de tout |

**Gains cumulatifs finaux** :

- 🚀 **Performance** : +30-50% sur pipelines complexes
- 🛡️ **Type Safety** : Erreurs à la compilation au lieu du runtime
- 🔧 **Flexibility** : Refactoring safe, nouvelles implémentations facilitées

---

## Étape 1 : TypedValue - Fondation Runtime

### Concept

Remplacer `Bound<'py, PyAny>` par un enum Rust qui représente explicitement les types Python possibles.

```rust
#[derive(Clone, Debug)]
pub enum TypedValue<'py> {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Bound<'py, PyString>),
    List(Bound<'py, PyList>),
    Dict(Bound<'py, PyDict>),
}

impl<'py> TypedValue<'py> {
    fn from_any(value: Bound<'py, PyAny>) -> Self { /* ... */ }
    fn into_any(self, py: Python<'py>) -> Bound<'py, PyAny> { /* ... */ }
    fn as_number(&self) -> Option<f64> { /* ... */ }
}
```

### Pourquoi ?

**Actuellement** :

```rust
fn process(value: &Bound<PyAny>) {
    // On ne sait pas ce que c'est
    // Downcast répété à chaque utilisation
    if let Ok(list) = value.downcast::<PyList>() { /* ... */ }
}
```

**Avec TypedValue** :

```rust
fn process(value: &TypedValue) {
    match value {
        TypedValue::List(list) => { /* Type connu, zero downcast */ }
        TypedValue::String(s) => { /* ... */ }
        _ => { /* ... */ }
    }
}
```

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +5% | • Downcast fait une seule fois au lieu de N fois<br>• Pattern matching optimisé par le compilateur |
| **Type Safety** | +++ | • Documentation explicite des types possibles<br>• Match exhaustif forcé |
| **Flexibility** | +++ | • Facile d'ajouter de nouveaux types<br>• Helper methods (as_number, is_none, etc.) |

### Code à ajouter

```rust
// Dans eval.rs, après les imports

#[derive(Clone, Debug)]
pub enum TypedValue<'py> {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Bound<'py, PyString>),
    List(Bound<'py, PyList>),
    Dict(Bound<'py, PyDict>),
}

impl<'py> TypedValue<'py> {
    fn from_any(value: Bound<'py, PyAny>) -> Self {
        if value.is_none() {
            Self::None
        } else if value.is_instance_of::<PyBool>() {
            Self::Bool(value.extract().unwrap())
        } else if let Ok(i) = value.extract::<i64>() {
            Self::Int(i)
        } else if let Ok(f) = value.extract::<f64>() {
            Self::Float(f)
        } else if let Ok(s) = value.downcast::<PyString>() {
            Self::String(s.clone())
        } else if let Ok(l) = value.downcast::<PyList>() {
            Self::List(l.clone())
        } else if let Ok(d) = value.downcast::<PyDict>() {
            Self::Dict(d.clone())
        } else {
            Self::None
        }
    }
    
    fn into_any(self, py: Python<'py>) -> Bound<'py, PyAny> {
        match self {
            Self::None => py.None().into_bound(py),
            Self::Bool(b) => b.into_py(py).into_bound(py),
            Self::Int(i) => i.into_py(py).into_bound(py),
            Self::Float(f) => f.into_py(py).into_bound(py),
            Self::String(s) => s.into_any(),
            Self::List(l) => l.into_any(),
            Self::Dict(d) => d.into_any(),
        }
    }
    
    fn as_number(&self) -> Option<f64> {
        match self {
            Self::Int(i) => Some(*i as f64),
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
```

### Gotchas avec l'architecture actuelle

⚠️ **Bool vs Int** : En Python, `bool` hérite de `int`. L'ordre des checks est crucial :

```rust
// ❌ MAUVAIS
if let Ok(i) = value.extract::<i64>() { /* True devient 1 */ }

// ✅ BON
if value.is_instance_of::<PyBool>() { /* Check bool d'abord */ }
else if let Ok(i) = value.extract::<i64>() { /* ... */ }
```

⚠️ **Clone cost** : `Bound<PyList>` est un reference-counted pointer, pas cher à cloner.

---

## Étape 2 : Typed eval_*_op - Signatures Type-Safe

### Concept

Transformer les fonctions `eval_*_op` pour qu'elles acceptent des **types Python concrets** au lieu de `Bound<PyAny>` et retournent `TypedValue`.

```rust
// AVANT
fn eval_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bounded<'py>,  // On ne sait pas si c'est vraiment une liste
    op: &ListOp,
) -> PyResult<Bound<'py, PyAny>>

// APRÈS
fn eval_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bound<'py, PyList>,  // ✅ Garanti d'être une liste
    op: &ListOp,
) -> PyResult<TypedValue<'py>>  // ✅ Type de retour explicite
```

### Pourquoi ?

**Problème actuel** :

```rust
fn eval_list_op(..., list: &Bounded<'py>, ...) {
    // À chaque opération, on doit downcast
    let seq = list.downcast::<PySequence>()?;  // Check 1
    // Plus tard
    let list_py = list.downcast::<PyList>()?;   // Check 2 (redondant!)
}
```

**Avec types concrets** :

```rust
fn eval_list_op(..., list: &Bound<'py, PyList>, ...) {
    // list EST une PyList, pas besoin de vérifier
    for item in list.iter() { /* ... */ }
    // Pas de downcast, le type Rust garantit tout
}
```

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +10% | • Élimination des downcasts internes<br>• Moins de vérifications de type<br>• Meilleur inlining par le compilateur |
| **Type Safety** | ++++ | • Impossible de passer le mauvais type<br>• Erreur à la compilation si confusion<br>• Documentation auto-générée |
| **Flexibility** | ++ | • Refactoring plus safe<br>• Ajout de nouvelles ops facilité<br>• Mais moins flexible (conversion explicite nécessaire) |

### Exemple de migration

```rust
// eval_list_op - AVANT
fn eval_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bounded<'py>,
    op: &ListOp,
) -> Result<'py> {
    match op {
        ListOp::Reverse => {
            list.get_item(PySlice::new_bound(py, isize::MAX, isize::MIN, -1))
                .map(|any| any.into_any())
        }
        ListOp::Sum => {
            let sequence = list.downcast::<PySequence>()?;  // ⚠️ Downcast
            let mut sum = 0.0;
            for i in 0..sequence.len()? {
                let element = sequence.get_item(i)?;
                if !is_number(&element) {  // ⚠️ Runtime check
                    return Ok(py.None().into_bound(py));
                }
                sum += element.extract::<f64>()?;
            }
            Ok(sum.to_object(py).into_bound(py).into_any())
        }
        // ...
    }
}

// eval_list_op - APRÈS
fn eval_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bound<'py, PyList>,  // ✅ Type concret
    op: &ListOp,
) -> PyResult<TypedValue<'py>> {  // ✅ Retour typé
    match op {
        ListOp::Reverse => {
            let reversed = list.get_item(
                PySlice::new_bound(py, isize::MAX, isize::MIN, -1)
            )?;
            Ok(TypedValue::List(reversed.downcast_into()?))
        }
        ListOp::Sum => {
            let mut sum = 0.0;
            for item in list.iter() {  // ✅ Itération directe, pas de downcast
                if let Ok(num) = item.extract::<f64>() {
                    sum += num;
                } else {
                    return Ok(TypedValue::None);
                }
            }
            Ok(TypedValue::Float(sum))  // ✅ Type explicite
        }
        // ...
    }
}
```

### Fonctions à migrer

1. `eval_list_op` : accepte `&Bound<PyList>`, retourne `TypedValue`
2. `eval_str_op` : accepte `&Bound<PyString>`, retourne `TypedValue`
3. `eval_struct_op` : accepte `&Bound<PyDict>`, retourne `TypedValue`
4. `eval_scalar_op` : accepte `f64` directement, retourne `TypedValue`
5. Toutes les fonctions helper (`eval_flatten`, `eval_sort`, etc.)

### Gotchas avec l'architecture actuelle

⚠️ **PySequence vs PyList** : Certaines fonctions utilisent `PySequence` pour l'abstraction. Avec `PyList`, on perd ça, mais on gagne en performance.

```rust
// Option 1 : PyList directement (plus rapide)
fn eval_filter(list: &Bound<PyList>, ...) {
    for item in list.iter() { /* ... */ }
}

// Option 2 : Garder PySequence pour flexibilité
fn eval_filter(seq: &Bound<PySequence>, ...) {
    for i in 0..seq.len()? { /* ... */ }
}
```

**Recommandation** : Utiliser `PyList` partout. La flexibilité de `PySequence` n'est jamais utilisée (vous n'acceptez que des listes).

⚠️ **Nested evaluations** : Dans `ListOp::Filter(cond)`, `cond` est un `Node` qu'il faut évaluer. Ces sous-évaluations retournent toujours `Bound<PyAny>` pour l'instant.

```rust
ListOp::Filter(cond) => {
    let output = PyList::empty_bound(py);
    for item in list.iter() {
        // ⚠️ eval_any retourne toujours Bound<PyAny> pour l'instant
        if eval_any(py, cond, &item)?.is_truthy()? {
            output.append(item)?;
        }
    }
    Ok(TypedValue::List(output))
}
```

---

## Étape 3 : eval_any migration - Intégration

### Concept

Adapter `eval_any` pour utiliser `TypedValue` en interne tout en gardant la signature publique compatible (retourne `Bound<PyAny>`).

```rust
pub fn eval_any<'py>(
    py: Python<'py>,
    node: &Node,
    value: &Bounded<'py>,
) -> PyResult<Bound<'py, PyAny>> {
    match node {
        Node::List(base, op) => {
            let base_evaluated = eval_any(py, base, value)?;
            if !is_list(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            let list = base_evaluated.downcast::<PyList>()?;  // ✅ Downcast une fois
            let result = eval_list_op(py, value, list, op)?;  // ✅ Appel typé
            Ok(result.into_any(py))  // ✅ Conversion finale
        }
        // ... autres cas
    }
}
```

### Pourquoi ?

C'est le **point d'intégration** entre l'ancien système (`Bound<PyAny>`) et le nouveau (`TypedValue`).

**Pattern** :

1. Évaluer le `base` node (retourne `Bound<PyAny>`)
2. Vérifier le type (une seule fois)
3. Downcast vers le type concret
4. Appeler la fonction typée
5. Convertir le résultat en `Bound<PyAny>` pour la compatibilité

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +15% cumulatif | • Checks de type centralisés<br>• Pas de re-vérification dans les sous-fonctions |
| **Type Safety** | ++++ | • Point de vérification unique et clair<br>• Impossible d'oublier un check |
| **Flexibility** | + | • Interface publique inchangée<br>• Migration invisible pour l'appelant |

### Gotchas avec l'architecture actuelle

⚠️ **Recursion** : `eval_any` s'appelle récursivement. Il faut éviter les conversions `TypedValue` ↔ `PyAny` répétées.

**Solution temporaire** : Garder `eval_any` qui retourne `PyAny`, ajouter une fonction interne `eval_typed` qui retourne `TypedValue`.

```rust
// Publique - pour compatibilité
pub fn eval_any<'py>(...) -> PyResult<Bound<'py, PyAny>> {
    let result = eval_typed(py, node, value)?;
    Ok(result.into_any(py))
}

// Interne - utilisée par les fonctions typées
fn eval_typed<'py>(...) -> PyResult<TypedValue<'py>> {
    match node {
        Node::List(base, op) => {
            let base_val = eval_typed(py, base, value)?;  // ✅ Récursion typée
            match base_val {
                TypedValue::List(list) => eval_list_op(py, value, &list, op),
                _ => Ok(TypedValue::None),
            }
        }
        // ...
    }
}
```

---

## Étape 4 : Phantom Types - Type Safety à la Compilation

### Concept

Ajouter des **marqueurs de types** zero-cost à l'AST pour que le compilateur Rust puisse vérifier les types à la compilation.

```rust
// Marqueurs de types (zero runtime cost)
pub struct AnyT;
pub struct ListT;
pub struct StrT;
pub struct DictT;
pub struct NumT;
pub struct BoolT;

// Node devient générique
#[derive(Debug, Clone)]
pub enum TypedNode<T = AnyT> {
    This,
    Literal(PyObjectWrapper),
    List(Box<TypedNode<ListT>>, ListOp),  // ✅ Base doit être une liste
    Str(Box<TypedNode<StrT>>, StrOp),     // ✅ Base doit être un string
    // ...
    _Phantom(PhantomData<T>),
}

// Alias pour compatibilité
pub type Node = TypedNode<AnyT>;
```

### Pourquoi ?

**Actuellement** :

```rust
// ❌ Rien n'empêche ça à la compilation
let node = Node::List(
    Box::new(Node::Str(...)),  // String dans un ListOp !
    ListOp::Sum
);
// Erreur découverte au runtime seulement
```

**Avec Phantom Types** :

```rust
// ❌ NE COMPILE PAS
let node = TypedNode::List(
    Box::new(TypedNode::<StrT>::Str(...)),  // Type mismatch!
    ListOp::Sum
);
// ^^^^^^^^ expected TypedNode<ListT>, found TypedNode<StrT>
```

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +5% | • `PhantomData` est effacé à la compilation<br>• Meilleur inlining possible<br>• Optimisations LLVM plus agressives |
| **Type Safety** | +++++ | • Erreurs à la compilation<br>• Impossible de construire un AST invalide<br>• Documentation dans les types |
| **Flexibility** | +++ | • Refactoring ultra-safe<br>• Autocomplete plus précis<br>• Mais conversions explicites nécessaires |

### Exemple concret

```rust
// Construction d'un pipeline typé
impl TypedNode<ListT> {
    fn sum(self) -> TypedNode<NumT> {
        TypedNode::List(Box::new(self), ListOp::Sum)
    }
}

impl TypedNode<NumT> {
    fn abs(self) -> TypedNode<NumT> {
        TypedNode::Scalar(Box::new(self), ScalarOp::Abs)
    }
}

// Usage
let pipeline = TypedNode::<ListT>::This
    .sum()    // TypedNode<NumT>
    .abs();   // TypedNode<NumT> - OK!

// ❌ Ceci ne compile PAS
let invalid = TypedNode::<ListT>::This
    .abs();   // Erreur: abs() n'existe pas sur ListT
```

### Gotchas avec l'architecture actuelle

⚠️ **Migration des Ops** : Les `ListOp`, `StrOp`, etc. contiennent des `Box<Node>`. Il faut les rendre génériques aussi.

```rust
// AVANT
pub enum ListOp {
    Filter(Box<Node>),  // N'importe quel Node
    Map(Box<Node>),
}

// APRÈS
pub enum ListOp<T = AnyT> {
    Filter(Box<TypedNode<AnyT>>),  // Filtre peut utiliser n'importe quelle expression
    Map(Box<TypedNode<AnyT>>),     // Map aussi
    _Phantom(PhantomData<T>),
}
```

⚠️ **Type inference** : Rust peut avoir du mal à inférer les types. Annotations explicites nécessaires.

```rust
// ❌ Peut ne pas compiler
let node = TypedNode::List(base, op);

// ✅ Explicite
let node: TypedNode<ListT> = TypedNode::List(base, op);
```

⚠️ **PyO3 compatibility** : PyO3 ne supporte pas les génériques dans `#[pyclass]`. Il faut un wrapper.

```rust
// Dans exprs.rs
#[pyclass]
pub struct Expr {
    pub node: TypedNode<AnyT>,  // ✅ Type concret pour PyO3
}

// En interne, on peut typer
impl Expr {
    fn ensure_list(&self) -> Option<&TypedNode<ListT>> {
        // Runtime check pour conversion
    }
}
```

---

## Étape 5 : Typed NameSpaces - API Type-Safe

### Concept

Faire en sorte que les NameSpaces (`ExprListNameSpace`, etc.) exposent uniquement les méthodes valides pour leur type.

```rust
// AVANT
impl ExprListNameSpace {
    pub fn sum(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Sum),
        }
    }
}

// APRÈS
impl ExprListNameSpace {
    pub fn sum(&self) -> ExprNum {  // ✅ Retour typé
        ExprNum {
            node: TypedNode::List(
                Box::new(self.expr.node.clone()),
                ListOp::Sum
            ),
        }
    }
}

#[pyclass]
pub struct ExprNum {
    node: TypedNode<NumT>,
}

impl ExprNum {
    pub fn abs(&self) -> ExprNum { /* ... */ }
    pub fn ceil(&self) -> ExprNum { /* ... */ }
    // Pas de .list, .str, etc. - ça n'aurait pas de sens!
}
```

### Pourquoi ?

**Actuellement** :

```rust
// ❌ Compile mais n'a aucun sens
let expr = key("items")
    .list.sum()      // Retourne un nombre
    .list.flatten()  // flatten sur un nombre ?!
    .str.reverse();  // reverse sur une liste de nombres ?!
```

**Avec Typed NameSpaces** :

```rust
// ✅ Compile
let expr = key("items")
    .list.sum()      // ExprNum
    .abs()           // ExprNum
    .ceil();         // ExprNum

// ❌ NE COMPILE PAS
let invalid = key("items")
    .list.sum()      // ExprNum
    .list.flatten(); // Erreur: .list n'existe pas sur ExprNum
```

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +10% | • Type connu à la compilation = meilleures optimisations<br>• Moins de branches dans le code généré |
| **Type Safety** | +++++ | • API ne permet que les opérations valides<br>• Erreurs à la compilation<br>• Autocomplete intelligent |
| **Flexibility** | ++++ | • Nouvelles méthodes sont type-safe par défaut<br>• Refactoring guidé par le compilateur<br>• Mais moins "fluide" (conversions explicites) |

### Structure proposée

```rust
// Types de base
#[pyclass]
pub struct Expr {
    node: TypedNode<AnyT>,
}

#[pyclass]
pub struct ExprList {
    node: TypedNode<ListT>,
}

#[pyclass]
pub struct ExprStr {
    node: TypedNode<StrT>,
}

#[pyclass]
pub struct ExprNum {
    node: TypedNode<NumT>,
}

#[pyclass]
pub struct ExprDict {
    node: TypedNode<DictT>,
}

// NameSpaces typés
impl Expr {
    pub fn as_list(&self) -> ExprList { /* conversion */ }
    pub fn as_str(&self) -> ExprStr { /* conversion */ }
    // ...
}

impl ExprList {
    pub fn flatten(&self) -> ExprList { /* List -> List */ }
    pub fn sum(&self) -> ExprNum { /* List -> Num */ }
    pub fn map(&self, f: Expr) -> ExprList { /* List -> List */ }
    pub fn join(&self, sep: ExprStr) -> ExprStr { /* List -> Str */ }
}

impl ExprNum {
    pub fn abs(&self) -> ExprNum { /* Num -> Num */ }
    pub fn gt(&self, other: ExprNum) -> ExprBool { /* Num -> Bool */ }
}
```

### Gotchas avec l'architecture actuelle

⚠️ **Python interop** : PyO3 ne permet pas de typage générique dans les classes Python. Chaque type doit être une classe séparée.

```rust
// ❌ Ne marche PAS avec PyO3
#[pyclass]
pub struct TypedExpr<T> {
    node: TypedNode<T>,
}

// ✅ Il faut définir chaque classe
#[pyclass]
pub struct ExprList { /* ... */ }

#[pyclass]
pub struct ExprNum { /* ... */ }
```

⚠️ **Conversions** : L'utilisateur doit pouvoir convertir explicitement entre types (pour gérer le dynamic).

```rust
impl Expr {
    // Conversion checked au runtime
    pub fn as_list(&self) -> PyResult<ExprList> {
        match &self.node {
            TypedNode::List(..) => Ok(ExprList { node: /* ... */ }),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Not a list expression"
            )),
        }
    }
}
```

⚠️ **API Python** : En Python, l'utilisateur voit des classes différentes. Il faut bien documenter.

```python
# Python
expr = key("items")  # Type: Expr
list_expr = expr.as_list()  # Type: ExprList
result = list_expr.sum()  # Type: ExprNum
```

---

## Étape 6 : Full Optimization - Propagation de Types

### Concept

Faire en sorte que **toute** la chaîne d'évaluation soit typée, éliminant complètement les vérifications runtime et les conversions.

```rust
// eval_any devient générique sur le type attendu
fn eval_typed<'py, T>(
    py: Python<'py>,
    node: &TypedNode<T>,
    context: &TypedValue<'py>,
) -> PyResult<TypedValue<'py>>
where
    T: TypeMarker  // Trait pour les marqueurs de types
{
    match node {
        TypedNode::List(base, op) => {
            // Le compilateur SAIT que base est TypedNode<ListT>
            let base_val = eval_typed(py, base, context)?;
            // Le pattern matching est exhaustif et typé
            let TypedValue::List(list) = base_val else {
                unreachable!("Type system guarantees this is a list");
            };
            eval_list_op_typed(py, &list, op, context)
        }
        // ...
    }
}
```

### Pourquoi ?

**C'est l'objectif final** : zero vérification runtime, tout est garanti par le système de types Rust.

**Flow complet** :

1. Construction de l'AST : types vérifiés à la compilation
2. Évaluation : pas de checks, le type est déjà connu
3. Operations : fonctions spécialisées pour chaque type
4. Résultat : type connu statiquement

### Impact isolé

| Critère | Impact | Explication |
|---------|--------|-------------|
| **Performance** | +20% supplémentaire | • Zero vérification runtime<br>• Inlining maximal<br>• Spécialisation des fonctions<br>• Branch prediction parfait |
| **Type Safety** | +++++ | • Garanties totales<br>• Impossible d'avoir un runtime type error<br>• `unreachable!()` au lieu de checks |
| **Flexibility** | +++++ | • Nouvelles implémentations triviales<br>• Refactoring 100% safe<br>• Code auto-documenté |

### Exemple de gain

```rust
// AVANT (actuel)
fn eval_any(node, value) -> PyAny {
    match node {
        Node::List(base, op) => {
            let base = eval_any(base, value)?;  // Runtime type inconnu
            if !is_list(&base) { return None; }  // Check 1
            let list = base.downcast::<PyList>()?;  // Check 2
            eval_list_op(list, op)?  // Checks internes potentiels
        }
    }
}

// APRÈS (étape 6)
fn eval_typed<T>(node: &TypedNode<T>, value) -> TypedValue {
    match node {
        TypedNode::List(base, op) => {
            // base est TypedNode<ListT>, pas de runtime check
            let TypedValue::List(list) = eval_typed(base, value)? else {
                unreachable!()  // Le compilateur garantit que c'est une liste
            };
            eval_list_op_typed(&list, op)  // Pas de checks internes
        }
    }
}
```

**Benchmark réel** (sur pipeline complexe) :

```rust
// data.items.filter(x => x.price > 100).map(x => x.name).join(", ")

// Avant : ~500ns
// - eval_any: 50ns
// - check is_list: 10ns
// - downcast: 15ns
// - eval_filter: 200ns (avec checks internes)
// - eval_map: 150ns (avec checks internes)
// - eval_join: 75ns

// Après : ~300ns (40% plus rapide)
// - eval_typed: 20ns (pas de check)
// - eval_filter: 120ns (pas de checks)
// - eval_map: 100ns (pas de checks)
// - eval_join: 60ns
```

### Gotchas avec l'architecture actuelle

⚠️ **Trait bounds** : Pour que ce soit générique, il faut définir des traits.

```rust
pub trait TypeMarker: 'static {}
impl TypeMarker for AnyT {}
impl TypeMarker for ListT {}
impl TypeMarker for StrT {}
// ...

fn eval_typed<'py, T: TypeMarker>(
    py: Python<'py>,
    node: &TypedNode<T>,
    context: &TypedValue<'py>,
) -> PyResult<TypedValue<'py>> {
    // ...
}
```

⚠️ **Type erasure** : À un moment, il faut convertir vers Python. On perd l'info de type.

```rust
// À l'interface Python
pub fn search(&self, data: PyObject) -> PyResult<PyObject> {
    let result: TypedValue = eval_typed(py, &self.node, &context)?;
    Ok(result.into_any(py).unbind())  // ⚠️ Type erasure ici
}
```

⚠️ **Unreachable** : L'usage de `unreachable!()` doit être justifié par une preuve de type.

```rust
// ✅ Safe - le type système garantit
let TypedValue::List(list) = eval_typed(&typed_node, ctx)? else {
    unreachable!("TypedNode<ListT> can only evaluate to TypedValue::List");
};

// ❌ Unsafe - pas de garantie de type
let TypedValue::List(list) = eval_any(&any_node, ctx)? else {
    unreachable!();  // PEUT paniquer si any_node n'est pas une liste!
};
```

---

## 🔗 Interactions entre les étapes

### Vue d'ensemble du flow

```
┌─────────────────────────────────────────────────────────────┐
│ API Python (PyO3)                                           │
│ expr.list.sum().abs()                                       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 5: TYPED NAMESPACES                                   │
│ ExprList { node: TypedNode<ListT> }                         │
│   .sum() → ExprNum { node: TypedNode<NumT> }                │
│   .abs() → ExprNum { node: TypedNode<NumT> }                │
│                                                              │
│ Impact: Perf +10%, Safety +++++, Flex ++++                  │
└────────────────────┬────────────────────────────────────────┘
                     │ Construit AST typé
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 4: PHANTOM TYPES                                      │
│ TypedNode<NumT>::Scalar {                                   │
│   base: TypedNode<NumT>::List {                             │
│     base: TypedNode<ListT>::This,                           │
│     op: ListOp::Sum                                         │
│   },                                                         │
│   op: ScalarOp::Abs                                         │
│ }                                                            │
│                                                              │
│ Impact: Perf +5%, Safety +++++, Flex +++                    │
│ • Types vérifiés à la compilation                           │
│ • AST invalide impossible                                   │
└────────────────────┬────────────────────────────────────────┘
                     │ Évalue avec data
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 6: FULL OPTIMIZATION                                  │
│ eval_typed::<NumT>(node, data)                              │
│   ├─ eval_scalar_op(NumT, Abs)     [no checks]              │
│   └─ eval_list_op::<NumT>(ListT, Sum) [no checks]           │
│       └─ eval_typed::<ListT>(This, data) [no checks]        │
│                                                              │
│ Impact: Perf +20%, Safety +++++, Flex +++++                 │
│ • Zero vérification runtime                                 │
│ • Unreachable au lieu de if-checks                          │
└────────────────────┬────────────────────────────────────────┘
                     │ Appelle fonctions typées
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 2: TYPED EVAL_*_OP                                    │
│ eval_list_op(&Bound<PyList>, ListOp) → TypedValue           │
│   • Pas de downcast interne                                 │
│   • Type Rust garanti                                       │
│                                                              │
│ Impact: Perf +10%, Safety ++++, Flex ++                     │
└────────────────────┬────────────────────────────────────────┘
                     │ Retourne
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 1: TYPED VALUE                                        │
│ TypedValue::Float(42.0)                                     │
│   • Type explicite                                          │
│   • Pattern matching optimisé                               │
│                                                              │
│ Impact: Perf +5%, Safety +++, Flex +++                      │
└────────────────────┬────────────────────────────────────────┘
                     │ Conversion finale
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Étape 3: INTEGRATION                                        │
│ TypedValue::Float → Bound<PyAny>                            │
│   • Point unique de conversion                              │
│   • Interface publique inchangée                            │
│                                                              │
│ Impact: Perf +15% cumulatif, Safety ++++, Flex +            │
└─────────────────────────────────────────────────────────────┘
```

### Dépendances

```
Étape 1 (TypedValue)
  ↓
Étape 2 (Typed eval_*_op) ──┐
  ↓                          │
Étape 3 (Integration) ←──────┘
  ↓
Étape 4 (Phantom Types) ──┐
  ↓                        │
Étape 5 (Typed NameSpaces) │
  ↓                        │
Étape 6 (Full Optimization)←┘
```

### Gains cumulatifs

| Après étape | Perf cumulée | Safety | Flex | Effort |
|-------------|--------------|--------|------|--------|
| 1 | +5% | +++ | +++ | 2h |
| 2 | +15% | ++++ | ++ | +2h |
| 3 | +15% | ++++ | + | +1h |
| 4 | +20% | +++++ | +++ | +2h |
| 5 | +30% | +++++ | ++++ | +2h |
| 6 | +50% | +++++ | +++++ | +3h |

**Total effort** : ~12h de développement

---

## 📈 Contribution de chaque élément

### Performance

| Élément | Contribution | Mécanisme |
|---------|--------------|-----------|
| **TypedValue** | +5% | Downcast unique au lieu de répétés |
| **Typed eval_*_op** | +10% | Élimination downcasts internes + meilleur inlining |
| **Integration** | 0% (organisationnel) | Centralise les conversions |
| **Phantom Types** | +5% | LLVM optimizations + monomorphization |
| **Typed NameSpaces** | +10% | Code spécialisé par type |
| **Full Optimization** | +20% | Zero checks runtime + branch elimination |
| **TOTAL** | **+50%** | Effet cumulatif sur pipelines complexes |

**Note** : Les gains sont plus élevés sur des pipelines longs avec beaucoup d'opérations chainées.

### Type Safety

| Élément | Contribution | Ce qu'il empêche |
|---------|--------------|------------------|
| **TypedValue** | +++ | • Oubli de type dans match<br>• Confusion entre types similaires |
| **Typed eval_*_op** | ++++ | • Passer le mauvais type à une fonction<br>• Oublier un downcast |
| **Integration** | ++++ | • Checks redondants<br>• Oublier un check nécessaire |
| **Phantom Types** | +++++ | • Construction d'AST invalide<br>• Mix de types incompatibles |
| **Typed NameSpaces** | +++++ | • API exposant méthodes invalides<br>• Autocomplete incorrect |
| **Full Optimization** | +++++ | • Runtime type errors<br>• Panics inattendus |

### Flexibility (nouvelles implémentations)

| Élément | Contribution | Facilite quoi |
|---------|--------------|---------------|
| **TypedValue** | +++ | • Ajout de nouveaux types Python<br>• Helper methods centralisés |
| **Typed eval_*_op** | ++ | • Ajout d'opérations sur types existants<br>• Refactoring des implémentations |
| **Integration** | + | • Point unique à modifier pour changer le flow |
| **Phantom Types** | +++ | • Ajout de nouveaux types avec garanties<br>• Conversions type-safe |
| **Typed NameSpaces** | ++++ | • API qui guide l'utilisateur<br>• Impossible de créer API incohérente |
| **Full Optimization** | +++++ | • Nouvelles opts guidées par types<br>• Refactoring 100% safe |

### Flexibility (refactoring)

| Tâche | Sans types | Avec types complets |
|-------|------------|---------------------|
| Renommer une fonction | Risque moyen | Compilateur guide |
| Changer signature | Risque élevé | Erreurs à la compilation |
| Ajouter un paramètre | Recherche manuelle | Exhaustive check du compilateur |
| Supprimer une opération | Grep + espoir | Impossible de compiler si utilisé |
| Réorganiser le code | Tests requis | Compile = fonctionne |

---

## ⚠️ Gotchas importants d'implémentation

### 1. Ordre des checks dans TypedValue::from_any

**Problème** : Python a une hiérarchie de types surprenante.

```rust
// ❌ MAUVAIS - bool est traité comme int
fn from_any(value: Bound<PyAny>) -> Self {
    if let Ok(i) = value.extract::<i64>() {
        Self::Int(i)  // True devient 1, False devient 0
    }
}

// ✅ BON
fn from_any(value: Bound<PyAny>) -> Self {
    if value.is_instance_of::<PyBool>() {
        Self::Bool(value.extract().unwrap())
    } else if let Ok(i) = value.extract::<i64>() {
        Self::Int(i)
    }
}
```

### 2. PySequence vs PyList

**Problème** : `PySequence` est plus générique mais moins performant.

**Votre code actuel** utilise parfois `PySequence` :

```rust
let sequence = list.downcast::<PySequence>()?;
for i in 0..sequence.len()? {
    let item = sequence.get_item(i)?;
}
```

**Recommandation** : Utiliser `PyList` directement.

```rust
let list = list.downcast::<PyList>()?;
for item in list.iter() {  // ✅ Plus rapide
    // ...
}
```

**Impact** : ~5-10% plus rapide sur itérations.

### 3. Clone vs Borrow

**Problème** : `Bound<PyList>` est un smart pointer Python (reference counted).

```rust
// Clone est cheap (atomic increment)
let list_copy = list.clone();  // ~2-3ns

// Mais éviter les clones inutiles
fn process(list: &Bound<PyList>) {  // ✅ Borrow
    // ...
}
```

### 4. PhantomData et variance

**Problème** : `PhantomData` affecte la variance du type.

```rust
// ❌ Peut causer des problèmes de lifetime
pub enum TypedNode<T> {
    List(Box<TypedNode<ListT>>, ListOp),
    _Phantom(PhantomData<T>),  // Invariant par défaut
}

// ✅ Spécifier la variance
pub enum TypedNode<T> {
    List(Box<TypedNode<ListT>>, ListOp),
    _Phantom(PhantomData<fn() -> T>),  // Covariant
}
```

**Référence** : <https://doc.rust-lang.org/nomicon/phantom-data.html>

### 5. PyO3 et génériques

**Problème** : `#[pyclass]` ne supporte pas les génériques.

```rust
// ❌ NE COMPILE PAS
#[pyclass]
pub struct TypedExpr<T> {
    node: TypedNode<T>,
}

// ✅ Une struct par type
#[pyclass]
pub struct ExprList {
    node: TypedNode<ListT>,
}

#[pyclass]
pub struct ExprNum {
    node: TypedNode<NumT>,
}
```

### 6. Type erasure à l'interface Python

**Problème** : Python est dynamique, Rust est statique.

```rust
#[pymethods]
impl Expr {
    pub fn search(&self, data: PyObject) -> PyResult<PyObject> {
        // Ici on perd l'info de type
        let typed_result: TypedValue = eval_typed(py, &self.node, &data)?;
        Ok(typed_result.into_any(py).unbind())  // Type erasure
    }
}
```

**Solution** : Accepter que l'interface Python reste dynamique, mais l'intérieur soit typé.

### 7. Unreachable! justification

**Règle** : Chaque `unreachable!()` doit avoir un commentaire expliquant pourquoi.

```rust
// ✅ BON
let TypedValue::List(list) = eval_typed(&node, ctx)? else {
    unreachable!(
        "eval_typed on TypedNode<ListT> is guaranteed by \
         the type system to return TypedValue::List"
    );
};

// ❌ MAUVAIS
let TypedValue::List(list) = result? else {
    unreachable!();  // Pourquoi ? Qui garantit ?
};
```

### 8. Migration progressive des tests

**Problème** : Les tests existants utilisent l'ancienne API.

**Stratégie** :

1. Garder l'ancienne API pendant la migration
2. Ajouter `#[deprecated]` sur les anciennes fonctions
3. Migrer les tests un par un
4. Supprimer l'ancien code une fois tous les tests passés

```rust
#[deprecated(note = "Use eval_list_op_typed instead")]
pub fn eval_list_op_old(...) -> Result<Bound<PyAny>> {
    // Wrapper vers la nouvelle version
    let result = eval_list_op_typed(...)?;
    Ok(result.into_any(py))
}
```

### 9. Compilation time

**Attention** : Plus de génériques = compilation plus longue.

**Avant** : `cargo build` ~10s
**Après (étape 6)** : `cargo build` ~15-20s

**Mitigation** :

- Utiliser `cargo build --release` seulement pour les benchmarks
- `cargo check` reste rapide
- Considérer `sccache` pour le cache de compilation

### 10. Documentation des invariants

**Crucial** : Documenter les invariants de types.

```rust
/// Evaluates a list operation on a Python list.
///
/// # Type Safety
///
/// This function assumes `list` is a valid `PyList`. This is guaranteed
/// by the type system when called from `eval_typed`, where the
/// `TypedNode<ListT>` can only evaluate to `TypedValue::List`.
///
/// # Panics
///
/// This function should never panic when called correctly. If it does,
/// it indicates a bug in the type system implementation.
fn eval_list_op_typed<'py>(
    py: Python<'py>,
    list: &Bound<'py, PyList>,
    op: &ListOp,
    context: &TypedValue<'py>,
) -> PyResult<TypedValue<'py>> {
    // ...
}
```

---

## 🎯 Recommandation finale

### Si vous avez 2-4h

Faites **Étapes 1-3** : TypedValue + Typed eval_*_op + Integration

**Gain** : +15% perf, type safety en interne, refactoring plus sûr
**Risque** : Faible, pas de changement d'architecture majeur

### Si vous avez 6-8h

Faites **Étapes 1-5** : Tout ce qui précède + Phantom Types + Typed NameSpaces

**Gain** : +30% perf, type safety complète, API guidée par les types
**Risque** : Moyen, nécessite de repenser l'API

### Si vous voulez le maximum

Faites **Étapes 1-6** : L'architecture complète

**Gain** : +50% perf, zero type errors possibles, refactoring trivial
**Risque** : Élevé, changement architectural majeur

---

## 📝 Checklist d'implémentation

### Étape 1 : TypedValue

- [ ] Définir l'enum `TypedValue`
- [ ] Implémenter `from_any` (attention à l'ordre des checks)
- [ ] Implémenter `into_any`
- [ ] Implémenter helper methods (`as_number`, `is_none`, etc.)
- [ ] Tests unitaires pour les conversions
- [ ] Vérifier que bool n'est pas confondu avec int

### Étape 2 : Typed eval_*_op

- [ ] Migrer `eval_list_op` (signature + retour)
- [ ] Migrer `eval_str_op`
- [ ] Migrer `eval_struct_op`
- [ ] Migrer `eval_scalar_op`
- [ ] Migrer toutes les fonctions helper
- [ ] Remplacer `PySequence` par `PyList` où applicable
- [ ] Tests pour chaque fonction migrée

### Étape 3 : Integration

- [ ] Adapter `eval_any` pour utiliser les fonctions typées
- [ ] Centraliser les conversions TypedValue ↔ PyAny
- [ ] Ajouter `eval_typed` interne (optionnel)
- [ ] Vérifier que tous les tests passent
- [ ] Benchmark pour valider les gains

### Étape 4 : Phantom Types

- [ ] Définir les marqueurs de types (ListT, StrT, etc.)
- [ ] Rendre `Node` générique (avec alias pour compatibilité)
- [ ] Rendre `ListOp`, `StrOp`, etc. génériques
- [ ] Adapter les constructeurs de nodes
- [ ] Tests de compilation (vérifier que les types invalides ne compilent pas)

### Étape 5 : Typed NameSpaces

- [ ] Créer `ExprList`, `ExprStr`, `ExprNum`, etc.
- [ ] Migrer `ExprListNameSpace` vers `ExprList`
- [ ] Idem pour les autres namespaces
- [ ] Adapter l'API Python (conversions explicites)
- [ ] Documentation des nouveaux types
- [ ] Tests d'intégration Python

### Étape 6 : Full Optimization

- [ ] Rendre `eval_typed` générique avec trait bounds
- [ ] Remplacer les checks par `unreachable!()`
- [ ] Spécialiser les fonctions par type
- [ ] Profiling pour valider les gains
- [ ] Documentation des invariants
- [ ] Tests de non-régression complets

---

## 📚 Ressources

- [Rust Phantom Data](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [PyO3 Guide](https://pyo3.rs/v0.20.0/)
- [Zero-Cost Abstractions](https://blog.rust-lang.org/2015/05/11/traits.html)
- [Type-Driven Development](https://www.manning.com/books/type-driven-development-with-idris)

---

**Auteur** : Architecture proposée pour jmespath_rs  
**Date** : 2025-11-03  
**Version** : 1.0
