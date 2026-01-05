# Legalis-RS: La Arquitectura de la Jurisprudencia Generativa

## Separando el Derecho y la Narrativa: Un Plano para "Gobernanza como Código"

---

**Autores**: Equipo de Desarrollo de Legalis-RS
**Versión**: 0.2.0
**Lenguaje**: Rust (Edition 2024)
**Licencia**: MIT / Apache 2.0

---

## Resumen

Este documento presenta **Legalis-RS**, un framework de Rust para separar y estructurar rigurosamente documentos legales en lenguaje natural en **lógica determinista (Code)** y **discreción judicial (Narrative)**.

Los sistemas jurídicos modernos contienen una mezcla de dominios susceptibles de automatización informática (requisitos de edad, umbrales de ingresos, cálculo de plazos) y dominios que requieren interpretación y juicio humano ("causa justa", "moral pública"). Los enfoques anteriores han dejado este límite ambiguo o han intentado una automatización excesiva que buscaba hacer todo computable.

Legalis-RS introduce un tipo de lógica de tres valores `LegalResult<T>` aprovechando el sistema de tipos de Rust para hacer explícito este límite a nivel de tipo. Esto permite un nuevo paradigma para la depuración legal, simulación y portabilidad internacional mientras previene la "autocracia algorítmica" en la era de la IA.

**Contribuciones Técnicas Principales**:
1. Lenguaje de Dominio Específico Legal (DSL) e implementación del analizador
2. Verificación formal con el solucionador Z3 SMT
3. Motor de simulación estilo ECS para predicción de impacto social
4. Generación de contratos inteligentes para más de 25 plataformas blockchain
5. Integración de Linked Open Data (RDF/TTL) para la web semántica
6. Implementaciones de sistemas legales para 4 países con adaptación de parámetros culturales (Soft ODA)

**Filosofía Central**: *"No todo debe ser computable."*

---

## 1. Introducción

### 1.1 Contexto: La Relación Entre el Derecho y la Computación

La famosa tesis de Lawrence Lessig "Code is Law" señaló que la arquitectura (código) en el ciberespacio tiene un poder regulatorio equivalente al derecho. Sin embargo, Legalis-RS invierte esto, adoptando un enfoque de "**El Derecho se convierte en Código**".

Codificar el derecho ofrece los siguientes beneficios:

- **Verificabilidad**: Detectar contradicciones lógicas en tiempo de compilación
- **Simulación**: Predecir impactos sociales antes de la aplicación
- **Interoperabilidad**: Convertir y comparar entre diferentes sistemas jurídicos
- **Transparencia**: Rastros de auditoría completos de los procesos de decisión legal

Sin embargo, hacer todas las leyes computables es peligroso tanto filosófica como prácticamente. El derecho contiene inherentemente dominios que requieren "juicio humano", y la automatización que ignora esto puede llevar a la "autocracia de IA".

### 1.2 Planteamiento del Problema: Desafíos del Procesamiento Legal en la Era de la IA

La tecnología legal moderna (LegalTech) enfrenta varios desafíos fundamentales:

1. **Manejo de la ambigüedad**: Muchos términos legales son intencionalmente vagos, presuponiendo interpretación caso por caso
2. **Dependencia del contexto**: La misma disposición puede interpretarse de manera diferente según el contexto social y cultural
3. **Cambio temporal**: Las leyes se modifican y derogan, requiriendo gestión de consistencia a través del tiempo
4. **Diferencias internacionales**: Los sistemas legales de cada país difieren desde sus fundamentos filosóficos

Los DSL legales existentes (Catala, L4, Stipula) han abordado algunos de estos desafíos, pero ninguno ha adoptado un enfoque que haga explícito el "límite entre la computabilidad y el juicio humano" en el sistema de tipos.

### 1.3 Propuesta: Separación de Computabilidad y Discreción Judicial

El núcleo de Legalis-RS es la introducción de lógica de tres valores a través del tipo `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Dominio Determinista] Resultados legales procesables automáticamente
    Deterministic(T),

    /// [Dominio Discrecional] Dominio que requiere juicio humano
    JudicialDiscretion {
        issue: String,           // El asunto en cuestión
        context_id: Uuid,        // Datos contextuales
        narrative_hint: Option<String>, // Opinión de referencia por LLM
    },

    /// [Colapso Lógico] Bug en la propia ley
    Void { reason: String },
}
```

Este tipo garantiza que el resultado del procesamiento legal siempre se clasifique en una de tres categorías. El sistema detiene el procesamiento al alcanzar `JudicialDiscretion` y delega el juicio a los humanos. Esto se convierte en una "fortaleza a nivel de tipo" contra la autocracia de IA.

### 1.4 Organización del Documento

El resto de este documento está organizado de la siguiente manera:

- **Sección 2**: Trabajo Relacionado
- **Sección 3**: Filosofía y Principios de Diseño
- **Sección 4**: Arquitectura del Sistema (estructura de 7 capas)
- **Sección 5**: Tecnologías Centrales
- **Sección 6**: Implementaciones Jurisdiccionales
- **Sección 7**: Estudios de Caso
- **Sección 8**: Especificación de API y Detalles Técnicos
- **Sección 9**: Evaluación
- **Sección 10**: Trabajo Futuro
- **Sección 11**: Conclusión

---

## 2. Trabajo Relacionado

### 2.1 Historia del Derecho Computacional

La relación entre el derecho y las computadoras se remonta al proyecto LARC (Legal Analysis and Research Computer) en la década de 1950.

| Era | Tecnología | Características |
|-----|------------|-----------------|
| 1950s | LARC | Primer sistema de recuperación de información legal |
| 1970s | Sistemas expertos tipo MYCIN | Razonamiento basado en reglas |
| 1980s | HYPO | Razonamiento basado en casos |
| 1990s | Estandarización XML/SGML | Estructuración de documentos legales |
| 2000s | Web Semántica | Representación del conocimiento legal basada en ontologías |
| 2010s | Aprendizaje Automático | Modelos de predicción legal |
| 2020s | LLM + Verificación Formal | Enfoque híbrido |

### 2.2 DSL Legales Existentes

#### Catala (Inria, Francia)
```
declaration scope AdultRights:
  context age content integer
  context has_rights content boolean

scope AdultRights:
  definition has_rights equals age >= 18
```
- **Características**: Programación literaria, basado en alcance, tipado fuerte
- **Limitaciones**: Sin marcado explícito de dominios discrecionales

#### L4 (Singapur)
```
RULE adult_voting
  PARTY citizen
  MUST vote
  IF age >= 18
```
- **Características**: Lógica deóntica (MUST/MAY/SHANT), razonamiento basado en reglas
- **Limitaciones**: Sin funcionalidad de simulación

#### Stipula (Universidad de Bolonia, Italia)
- **Características**: Orientado a contratos inteligentes, máquinas de estado, modelo de partes/activos
- **Limitaciones**: Sin verificación formal

### 2.3 Posicionamiento de Este Proyecto

Legalis-RS extiende la investigación existente de las siguientes maneras:

1. **Marcado de discreción a nivel de tipo**: Lógica de tres valores vía `LegalResult<T>`
2. **Arquitectura integrada**: Pipeline Parse→Verify→Simulate→Output
3. **Interoperabilidad multi-formato**: Conversión con Catala/L4/Stipula/Akoma Ntoso
4. **Diseño de internacionalización**: Adaptación de parámetros culturales (Soft ODA)
5. **Integración blockchain**: Generación de contratos inteligentes para más de 25 plataformas

---

## 3. Filosofía y Principios de Diseño

### 3.1 "Gobernanza como Código, Justicia como Narrativa"

El eslogan de Legalis-RS refleja la diferencia esencial entre gobernanza y justicia:

- **Gobernanza**: Aplicación de reglas, cumplimiento de procedimientos, determinación de elegibilidad → **Codificable**
- **Justicia**: Realización de equidad, interpretación contextual, juicio de valor → **Narrada como historia**

Esta distinción corresponde a la distinción entre "reglas" y "principios" (Dworkin) en la filosofía del derecho, o entre "justicia formal" y "justicia sustantiva".

### 3.2 Diseño de Lógica de Tres Valores

Los tres valores de `LegalResult<T>` corresponden a los siguientes conceptos de filosofía del derecho:

| Tipo | Concepto de Filosofía del Derecho | Agente de Procesamiento |
|------|-----------------------------------|------------------------|
| `Deterministic(T)` | Reglas aplicables mecánicamente | Computadora |
| `JudicialDiscretion` | Principios que requieren interpretación | Humano |
| `Void` | Lagunas legales/contradicciones | Legislador (necesita corrección) |

### 3.3 "No todo debe ser computable"

Contra la tentación de hacer todo computable, Legalis-RS dice claramente "No". Los siguientes dominios están intencionalmente diseñados como no computables:

1. **Causa justa**
2. **Orden público y moral**
3. **Buena fe**
4. **Razonabilidad**

### 3.4 Prevención de la Autocracia de IA

Legalis-RS previene la autocracia de IA a través de los siguientes mecanismos:

1. **Parada forzada por tipo**: Parada automática al alcanzar `JudicialDiscretion`
2. **Rastros de auditoría obligatorios**: Registro de todos los procesos de decisión
3. **Explicabilidad**: Salida estructurada de las razones de decisión
4. **Bucle humano garantizado**: Los humanos siempre toman decisiones finales en dominios discrecionales

---

## 4. Arquitectura del Sistema

### 4.1 Visión General de la Arquitectura de 7 Capas

Legalis-RS consta de las siguientes 7 capas:

```
┌─────────────────────────────────────────────────────────┐
│                  Capa de Infraestructura                 │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                      Capa de Salida                      │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│               Capa de Interoperabilidad                  │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│              Capa de Internacionalización                │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│            Capa de Simulación y Análisis                 │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                  Capa de Inteligencia                    │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Capa Central                        │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Capa Central

#### legalis-core
El crate que implementa el núcleo filosófico del proyecto.

**Definiciones de Tipos Principales**:
- `LegalResult<T>`: Tipo de lógica de tres valores
- `Statute`: Representación básica de leyes
- `Condition`: Expresiones de condición (AND/OR/NOT, edad, ingresos, etc.)
- `Effect`: Efectos legales (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Analizador para el lenguaje de dominio específico legal.

**Ejemplo de Sintaxis DSL**:
```
STATUTE adult-voting: "Derechos de Voto de Adultos" {
    JURISDICTION "ES"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Derechos de voto"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "La determinación de capacidad mental requiere diagnóstico médico"
}
```

### 4.3 Capa de Inteligencia

#### legalis-llm
Capa de abstracción de proveedores LLM.

**Proveedores Soportados**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- LLM Local

#### legalis-verifier
Motor de verificación formal.

**Objetivos de Verificación**:
- Detección de referencias circulares
- Detección de leyes inalcanzables (Dead Statute)
- Detección de contradicciones lógicas
- Verificación de conflictos constitucionales
- Análisis de ambigüedad

### 4.4 Capa de Simulación

#### legalis-sim
Motor de simulación estilo ECS.

**Características**:
- Simulación basada en población (soporta millones de agentes)
- Simulación Monte Carlo
- Análisis de sensibilidad
- Pruebas A/B
- Aceleración GPU (CUDA/OpenCL/WebGPU)

### 4.5 Capa de Internacionalización

#### legalis-i18n
Soporte multi-idioma y multi-jurisdicción.

**Jurisdicciones Soportadas**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG

### 4.6 Capa de Salida

#### legalis-chain
Generación de contratos inteligentes.

**Plataformas Soportadas (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm
- Otros: TON FunC, Algorand Teal, Fuel Sway, Clarity, Noir, Leo, Circom

**Restricción**: Solo `Deterministic` puede convertirse (`JudicialDiscretion` no puede convertirse)

#### legalis-lod
Salida Linked Open Data.

**Ontologías Soportadas**:
- ELI (European Legislation Identifier)
- FaBiO
- LKIF-Core
- Akoma Ntoso
- Dublin Core
- SKOS

---

## 5. Tecnologías Centrales

### 5.1 DSL Legal

**Estructura Básica**:
```
STATUTE <id>: "<title>" {
    [JURISDICTION "<jurisdiction>"]
    [VERSION <number>]
    [EFFECTIVE_DATE <date>]
    [EXPIRY_DATE <date>]

    WHEN <condition>
    THEN <effect>

    [EXCEPTION WHEN <condition>]
    [DISCRETION "<description>"]

    [AMENDMENT <statute-id>]
    [SUPERSEDES <statute-id>]
}
```

### 5.2 Tipo LegalResult<T> y Valores de Verdad Parciales

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion {
        issue: String,
        context_id: Uuid,
        narrative_hint: Option<String>,
    },
    Void { reason: String },
}

pub enum PartialBool {
    True,
    False,
    Unknown,      // Información insuficiente
    Contradiction, // Contradicción
}
```

### 5.3 Verificación Formal con el Solucionador Z3 SMT

**Objetivos de Verificación**:
1. Referencias circulares
2. Leyes inalcanzables
3. Contradicciones lógicas
4. Conflictos constitucionales

### 5.4 Motor de Simulación Estilo ECS

El motor de simulación adopta el patrón Entity-Component-System (ECS):
- **Entity**: Agentes ciudadanos
- **Component**: Atributos (edad, ingresos, residencia, etc.)
- **System**: Lógica de aplicación de leyes

### 5.5 Generación de Contratos Inteligentes

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AdultVotingRights {
    struct Citizen {
        uint256 age;
        bool hasCitizenship;
    }

    function isEligible(Citizen memory citizen)
        public pure returns (bool)
    {
        return citizen.age >= 18 && citizen.hasCitizenship;
    }
}
```

---

## 6. Implementaciones Jurisdiccionales

### 6.1 Sistema Legal Japonés

El crate legalis-jp proporciona una representación estructurada de la Constitución de Japón.

### 6.2 Alemania, Francia, EE.UU. (Planificado)

| Jurisdicción | Estado | Áreas de Enfoque |
|--------------|--------|------------------|
| Alemania (DE) | En desarrollo | BGB, GG |
| Francia (FR) | En desarrollo | Code civil, Constitución |
| EE.UU. (US) | En desarrollo | UCC, Constitución, Jurisprudencia |

### 6.3 Adaptación de Parámetros Culturales (Soft ODA)

Los siguientes parámetros culturales se consideran en la portabilidad internacional del sistema legal:

1. **Sistema legal**: Civil law vs Common law vs Religious law
2. **Estructura del idioma**: Traducibilidad de términos legales
3. **Normas sociales**: Tabúes, costumbres, restricciones religiosas
4. **Estructura administrativa**: Centralizado vs Federal
5. **Sistema judicial**: Jurado vs Jueces profesionales

---

## 7. Estudios de Caso

### 7.1 Sistema de Determinación de Elegibilidad de Bienestar

**Resultados**:
- **Decisiones deterministas**: 85% de los casos
- **JudicialDiscretion**: 15% de los casos (juicios sobre "urgencia", "necesidad genuina", etc.)

### 7.2 Simulación del Artículo 709 del Código Civil (Agravio)

**Escenarios de Prueba**:
1. Agravio intencional claro → `Deterministic(Liable)`
2. Agravio por negligencia → `Deterministic(Liable)`
3. Caso límite → `JudicialDiscretion`
4. Sin agravio → `Deterministic(NotLiable)`
5. Sin causalidad → `Deterministic(NotLiable)`

### 7.3 Análisis Comparativo de Derecho de Agravio de 4 Países

| País | Código | Características |
|------|--------|-----------------|
| Japón | Código Civil Art. 709 | Cláusula general (amplia discreción) |
| Alemania | BGB §823/§826 | Intereses protegidos enumerados |
| Francia | Code civil Art. 1240 | Abstracción máxima |
| EE.UU. | Jurisprudencia | Tipificado (Battery, etc.) |

---

## 8. Especificación de API y Detalles Técnicos

### 8.1 Tipos y Traits Principales

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}

pub struct Statute {
    pub id: String,
    pub title: String,
    pub primary_effect: Effect,
    pub preconditions: Vec<Condition>,
    pub jurisdiction: String,
    pub temporal_validity: TemporalValidity,
}
```

### 8.2 Sistema de Comandos CLI

```bash
# Analizar
legalis parse <file.dsl> [--format json|yaml]

# Verificar
legalis verify <file.dsl> [--strict]

# Simular
legalis simulate <file.dsl> --population 1000

# Visualizar
legalis visualize <file.dsl> --output tree.svg

# Exportar
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Evaluación

### 9.1 Benchmarks de Rendimiento

| Operación | Objetivo | Tiempo |
|-----------|----------|--------|
| Análisis DSL | 100 leyes | 15ms |
| Verificación | 100 leyes | 250ms |
| Simulación | 10,000 agentes | 1.2s |
| Simulación | 100,000 agentes | 8.5s |
| Generación de contrato inteligente | 1 ley | 45ms |
| Exportación RDF | 100 leyes | 120ms |

### 9.2 Calidad del Código

- **Cobertura de pruebas**: Pruebas de integración, pruebas de propiedades, pruebas de snapshot
- **Análisis estático**: Clippy (política de cero warnings)
- **Documentación**: rustdoc para todas las APIs públicas

---

## 10. Trabajo Futuro

### 10.1 Frontend Web UI
- Dashboard basado en React
- Visualización de simulación en tiempo real
- Características de edición colaborativa

### 10.2 Extensión VS Code
- Resaltado de sintaxis DSL
- Verificación en tiempo real
- Autocompletado

### 10.3 Integración Jupyter Notebook
- Bindings de Python vía PyO3
- Análisis interactivo
- Widgets de visualización

### 10.4 Jurisdicciones Adicionales
- Derecho de la UE (integración EURLex)
- Derecho internacional (tratados, acuerdos)
- Derecho religioso (jurisprudencia islámica)

---

## 11. Conclusión

Legalis-RS presenta un nuevo enfoque para codificar el derecho haciendo explícito el "límite entre la computabilidad y el juicio humano" en el sistema de tipos.

**Logros Principales**:

1. **Fundamento filosófico**: "Gobernanza como Código, Justicia como Narrativa"
2. **Sistema de tipos**: Lógica de tres valores vía `LegalResult<T>`
3. **Arquitectura integrada**: Diseño integral con 7 capas y 16 crates
4. **Implementación**: Aproximadamente 450,000 líneas de código Rust
5. **Verificación**: Integración del solucionador Z3 SMT
6. **Simulación**: Motor estilo ECS (soporte de aceleración GPU)
7. **Salida**: 25+ blockchains, RDF/TTL, múltiples formatos

**Filosofía Central**: *"No todo debe ser computable."*

No la automatización completa del derecho, sino la clara separación de dominios que deben automatizarse de dominios que requieren juicio humano. Esta es la arquitectura de la "jurisprudencia generativa" que Legalis-RS pretende.

---

## Referencias

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. Governatori, G., & Shams, Z. (2019). L4: Legal Language and Logic for Law. *JURIX 2019*.
5. Azzopardi, S., & Pace, G. J. (2018). Stipula: A domain-specific language for legal contracts. *JURIX 2018*.
6. Palmirani, M., & Vitali, F. (2011). Akoma-Ntoso for Legal Documents. *Legislative XML for the Semantic Web*.
7. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

## Apéndice

### A. Especificación de Gramática DSL

```ebnf
statute      = "STATUTE" identifier ":" string "{" body "}" ;
body         = { metadata } when_clause then_clause { exception } { discretion } ;
metadata     = jurisdiction | version | effective_date | expiry_date ;
jurisdiction = "JURISDICTION" string ;
version      = "VERSION" number ;
when_clause  = "WHEN" condition ;
then_clause  = "THEN" effect ;
exception    = "EXCEPTION" "WHEN" condition ;
discretion   = "DISCRETION" string ;
```

### B. Lista de Definiciones de Tipos

Para definiciones completas de tipos principales, ver `crates/legalis-core/src/lib.rs`.

### C. Opciones de Configuración

```toml
[legalis]
default_jurisdiction = "ES"
enable_z3 = true
enable_gpu = false
cache_dir = "~/.legalis/cache"
log_level = "info"

[api]
port = 8080
enable_graphql = true
enable_auth = true
rate_limit = 100

[simulation]
max_agents = 1000000
parallel_workers = 8
```

---

*"Code is Law," dicen, pero nosotros adoptamos el enfoque de "El Derecho se convierte en Código". Sin embargo, incorporamos un tipo llamado 'Humanidad' en ese código.*

---

**Equipo de Desarrollo Legalis-RS**
Versión 0.2.0 | 2024
