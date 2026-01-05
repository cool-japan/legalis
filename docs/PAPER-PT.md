# Legalis-RS: A Arquitetura da Jurisprudência Generativa

## Separando Direito e Narrativa: Um Modelo para "Governança como Código"

---

**Autores**: Equipe de Desenvolvimento Legalis-RS
**Versão**: 0.2.0
**Linguagem**: Rust (Edição 2024)
**Licença**: MIT / Apache 2.0

---

## Resumo

Este artigo apresenta o **Legalis-RS**, um framework em Rust para separar e estruturar rigorosamente documentos jurídicos em linguagem natural em **lógica determinística (Código)** e **discricionariedade judicial (Narrativa)**.

Os sistemas jurídicos modernos contêm uma mistura de domínios passíveis de automação computacional (requisitos de idade, limites de renda, cálculos de prazos) e domínios que requerem interpretação e julgamento humano ("justa causa", "moral pública"). Abordagens anteriores deixaram esse limite ambíguo ou tentaram automatização excessiva que buscava tornar tudo computável.

O Legalis-RS introduz um tipo de lógica trivalente `LegalResult<T>` aproveitando o sistema de tipos do Rust para tornar esse limite explícito no nível de tipos. Isso permite um novo paradigma para depuração, simulação e portabilidade internacional de leis, prevenindo a "autocracia algorítmica" na era da IA.

**Principais Contribuições Técnicas**:
1. Linguagem de Domínio Específico (DSL) jurídica e implementação de parser
2. Verificação formal com solucionador SMT Z3
3. Motor de simulação estilo ECS para previsão de impacto social
4. Geração de contratos inteligentes para mais de 25 plataformas blockchain
5. Integração de Dados Abertos Vinculados (RDF/TTL) para a web semântica
6. Implementações de sistemas jurídicos para 4 países com adaptação de parâmetros culturais (Soft ODA)

**Filosofia Central**: *"Nem tudo deve ser computável."*

---

## 1. Introdução

### 1.1 Contexto: A Relação Entre Direito e Computação

A famosa tese de Lawrence Lessig "Code is Law" (Código é Lei) apontou que a arquitetura (código) no ciberespaço tem poder regulatório equivalente à lei. No entanto, o Legalis-RS inverte isso, adotando uma abordagem de "**Lei se torna Código**".

Codificar a lei oferece os seguintes benefícios:

- **Verificabilidade**: Detectar contradições lógicas em tempo de compilação
- **Simulação**: Prever impactos sociais antes da aplicação
- **Interoperabilidade**: Converter e comparar entre diferentes sistemas jurídicos
- **Transparência**: Trilhas de auditoria completas dos processos de decisão jurídica

No entanto, tornar todas as leis computáveis é perigoso tanto filosoficamente quanto praticamente. A lei inerentemente contém domínios que requerem "julgamento humano", e a automação que ignora isso pode levar à "autocracia da IA".

### 1.2 Formulação do Problema: Desafios do Processamento Jurídico na Era da IA

A tecnologia jurídica moderna (LegalTech) enfrenta vários desafios fundamentais:

1. **Tratamento de Ambiguidade**: Muitos termos jurídicos são intencionalmente vagos, pressupondo interpretação caso a caso
2. **Dependência de Contexto**: A mesma disposição pode ser interpretada de forma diferente dependendo do contexto social e cultural
3. **Mudança Temporal**: As leis são alteradas e revogadas, exigindo gerenciamento de consistência ao longo do tempo
4. **Diferenças Internacionais**: Os sistemas jurídicos de cada país diferem desde suas bases filosóficas

DSLs jurídicas existentes (Catala, L4, Stipula) abordaram alguns desses desafios, mas nenhuma adotou uma abordagem que torna o "limite entre computabilidade e julgamento humano" explícito no sistema de tipos.

### 1.3 Proposta: Separação de Computabilidade e Discricionariedade Judicial

O núcleo do Legalis-RS é a introdução de lógica trivalente através do tipo `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Domínio Determinístico] Resultados jurídicos processáveis automaticamente
    Deterministic(T),

    /// [Domínio Discricionário] Domínio que requer julgamento humano
    JudicialDiscretion {
        issue: String,           // A questão em análise
        context_id: Uuid,        // Dados contextuais
        narrative_hint: Option<String>, // Opinião de referência por LLM
    },

    /// [Colapso Lógico] Bug na própria lei
    Void { reason: String },
}
```

Este tipo garante que o resultado do processamento jurídico seja sempre classificado em uma de três categorias. O sistema para o processamento ao alcançar `JudicialDiscretion` e delega o julgamento aos humanos. Isso se torna um "baluarte no nível de tipos" contra a autocracia da IA.

### 1.4 Organização do Artigo

O restante deste artigo está organizado da seguinte forma:

- **Seção 2**: Trabalhos Relacionados (História do Direito Computacional e DSLs existentes)
- **Seção 3**: Filosofia e Princípios de Design
- **Seção 4**: Arquitetura do Sistema (estrutura de 7 camadas)
- **Seção 5**: Tecnologias Centrais (DSL, verificação, simulação)
- **Seção 6**: Implementações Jurisdicionais (foco no direito japonês)
- **Seção 7**: Estudos de Caso
- **Seção 8**: Especificação de API e Detalhes Técnicos
- **Seção 9**: Avaliação
- **Seção 10**: Trabalhos Futuros
- **Seção 11**: Conclusão

---

## 2. Trabalhos Relacionados

### 2.1 História do Direito Computacional

A relação entre direito e computadores remonta ao projeto LARC (Legal Analysis and Research Computer) na década de 1950. Desde então, evoluiu através de sistemas especialistas, sistemas baseados em regras e abordagens modernas de aprendizado de máquina.

Marcos principais:

| Era | Tecnologia | Características |
|-----|------------|-----------------|
| Anos 1950 | LARC | Primeiro sistema de recuperação de informação jurídica |
| Anos 1970 | Sistemas especialistas tipo MYCIN | Raciocínio baseado em regras |
| Anos 1980 | HYPO | Raciocínio baseado em casos |
| Anos 1990 | Padronização XML/SGML | Estruturação de documentos jurídicos |
| Anos 2000 | Web Semântica | Representação de conhecimento jurídico baseada em ontologia |
| Anos 2010 | Aprendizado de Máquina | Modelos de previsão jurídica |
| Anos 2020 | LLM + Verificação Formal | Abordagem híbrida |

### 2.2 DSLs Jurídicas Existentes

Várias linguagens de domínio específico jurídico foram desenvolvidas:

#### Catala (Inria, França)
```
declaration scope AdultRights:
  context age content integer
  context has_rights content boolean

scope AdultRights:
  definition has_rights equals age >= 18
```
- **Características**: Programação literária, baseada em escopo, tipagem forte
- **Limitações**: Sem marcação explícita de domínios discricionários

#### L4 (Singapura)
```
RULE adult_voting
  PARTY citizen
  MUST vote
  IF age >= 18
```
- **Características**: Lógica deôntica (MUST/MAY/SHANT), raciocínio baseado em regras
- **Limitações**: Sem funcionalidade de simulação

#### Stipula (Universidade de Bolonha, Itália)
```
agreement AdultRights(citizen) {
  state: pending
  asset: rights

  citizen triggers accept when age >= 18 {
    transfer rights to citizen
    state: granted
  }
}
```
- **Características**: Orientado a contratos inteligentes, máquinas de estado, modelo parte/ativo
- **Limitações**: Sem verificação formal

### 2.3 Verificação Formal e Direito

A verificação formal do direito tem sido estudada principalmente usando model checking e solucionadores SMT. Solucionadores SMT como Z3 (Microsoft Research) e CVC5 podem determinar a satisfatibilidade de lógica proposicional e de predicados, detectando contradições lógicas nas leis.

No entanto, a pesquisa existente focou principalmente na consistência interna de leis individuais, com investigação limitada de interações entre múltiplas leis ou consistência com o direito constitucional.

### 2.4 Posicionamento Deste Projeto

O Legalis-RS estende a pesquisa existente das seguintes formas:

1. **Marcação de discricionariedade no nível de tipos**: Lógica trivalente via `LegalResult<T>`
2. **Arquitetura integrada**: Pipeline Parse→Verify→Simulate→Output
3. **Interoperabilidade multi-formato**: Conversão com Catala/L4/Stipula/Akoma Ntoso
4. **Design de internacionalização**: Adaptação de parâmetros culturais (Soft ODA)
5. **Integração blockchain**: Geração de contratos inteligentes para mais de 25 plataformas

---

## 3. Filosofia e Princípios de Design

### 3.1 "Governança como Código, Justiça como Narrativa"

O slogan do Legalis-RS reflete a diferença essencial entre governança e justiça:

- **Governança**: Aplicação de regras, conformidade processual, determinação de elegibilidade → **Codificável**
- **Justiça**: Realização de equidade, interpretação contextual, julgamento de valor → **Contada como narrativa**

Esta distinção corresponde à distinção entre "regras" e "princípios" (Dworkin) na filosofia do direito, ou entre "justiça formal" e "justiça substantiva".

### 3.2 Design da Lógica Trivalente

Os três valores de `LegalResult<T>` correspondem aos seguintes conceitos da filosofia jurídica:

| Tipo | Conceito da Filosofia Jurídica | Agente de Processamento |
|------|-------------------------------|------------------------|
| `Deterministic(T)` | Regras mecanicamente aplicáveis | Computador |
| `JudicialDiscretion` | Princípios que requerem interpretação | Humano |
| `Void` | Lacunas/contradições legais | Legislador (correção necessária) |

Este design torna o sistema sempre explícito sobre "quem deve fazer o julgamento".

### 3.3 "Nem tudo deve ser computável"

Contra a tentação de tornar tudo computável, o Legalis-RS diz claramente "Não". Os seguintes domínios são intencionalmente projetados como não-computáveis:

1. **Justa causa**
2. **Ordem pública e moral**
3. **Boa-fé**
4. **Razoabilidade**

Estes conceitos dependem do contexto social e cultural e são construídos como "narrativas" caso a caso. LLMs podem fornecer "opiniões de referência" sobre estes, mas não têm autoridade de decisão.

### 3.4 Prevenindo a Autocracia da IA

O Legalis-RS previne a autocracia da IA através dos seguintes mecanismos:

1. **Parada forçada por tipo**: Parada automática ao alcançar `JudicialDiscretion`
2. **Trilhas de auditoria obrigatórias**: Registro de todos os processos de decisão
3. **Explicabilidade**: Saída estruturada da fundamentação das decisões
4. **Circuito humano garantido**: Humanos sempre tomam decisões finais em domínios discricionários

---

## 4. Arquitetura do Sistema

### 4.1 Visão Geral da Arquitetura de 7 Camadas

O Legalis-RS consiste nas seguintes 7 camadas:

```
┌─────────────────────────────────────────────────────────┐
│                  Camada de Infraestrutura               │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                     Camada de Saída                     │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│               Camada de Interoperabilidade              │
│                    (legalis-interop)                    │
├─────────────────────────────────────────────────────────┤
│              Camada de Internacionalização              │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│              Camada de Simulação e Análise              │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                  Camada de Inteligência                 │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Camada Central                     │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Camada Central

#### legalis-core
O crate que implementa o núcleo filosófico do projeto.

**Definições de Tipos Principais**:
- `LegalResult<T>`: Tipo de lógica trivalente
- `Statute`: Representação básica de leis
- `Condition`: Expressões de condição (AND/OR/NOT, idade, renda, etc.)
- `Effect`: Efeitos jurídicos (Grant/Revoke/Obligation/Prohibition)
- `EvaluationContext`: Trait para avaliação de condições

**Estrutura de Módulos**:
- `case_law`: Gestão de jurisprudência
- `temporal`: Gestão temporal bi-temporal (relações de Allen)
- `formal_methods`: Exportação Coq/Lean4/TLA+/Alloy/SMTLIB
- `knowledge_graph`: Grafo de conhecimento jurídico
- `distributed`: Nós distribuídos, sharding

#### legalis-dsl
Parser para a linguagem de domínio específico jurídico.

**Exemplo de Sintaxe DSL**:
```
STATUTE adult-voting: "Direitos de Voto para Adultos" {
    JURISDICTION "BR"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Direitos de voto"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "Determinação de capacidade mental requer diagnóstico médico"
}
```

**Sintaxe Suportada**:
- STATUTE / WHEN / THEN / EXCEPTION
- AMENDMENT / SUPERSEDES
- AND / OR / NOT (com parênteses)
- Metadados (JURISDICTION, VERSION, EFFECTIVE_DATE, EXPIRY_DATE)

#### legalis-registry
Registro legal e controle de versão.

**Características**:
- Controle de versão estilo Git
- Organização baseada em tags
- Ancoragem em blockchain (Ethereum, Bitcoin, OpenTimestamps)
- Busca vetorial
- Registro distribuído

### 4.3 Camada de Inteligência

#### legalis-llm
Camada de abstração de provedores LLM.

**Provedores Suportados**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- LLM Local

**Características Principais**:
- `LawCompiler`: Conversão linguagem natural → lei estruturada
- `legal_prompting`: Prompts específicos para direito
- `legal_agents`: Framework de agentes
- `rag`: Retrieval Augmented Generation
- `safety_compliance`: Verificação de segurança

#### legalis-verifier
Motor de verificação formal.

**Alvos de Verificação**:
- Detecção de referência circular
- Detecção de lei inalcançável (Dead Statute)
- Detecção de contradição lógica
- Verificação de conflito constitucional
- Análise de ambiguidade

**Tecnologia**: Solucionador SMT Z3 (opcional)

### 4.4 Camada de Simulação

#### legalis-sim
Motor de simulação estilo ECS.

**Características**:
- Simulação baseada em população (suporta milhões de agentes)
- Simulação Monte Carlo
- Análise de sensibilidade
- Teste A/B
- Aceleração GPU (CUDA/OpenCL/WebGPU)

**Modelagem Econômica**:
- Previsão de receita tributária
- Análise de custo de conformidade
- Análise de custo-efetividade

#### legalis-diff
Detecção de mudanças legais e análise de impacto.

**Características**:
- Diff estrutural
- Diff semântico
- Classificação de mudanças
- Análise de compatibilidade retroativa/prospectiva
- Funcionalidade de rollback

### 4.5 Camada de Internacionalização

#### legalis-i18n
Suporte multi-idioma e multi-jurisdição.

**Jurisdições Suportadas**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG

**Características**:
- Códigos de idioma ISO 639-1
- Formato de mensagem ICU
- Regras de plural
- Formatação de data/hora/moeda/número

#### legalis-porting
Portabilidade entre sistemas jurídicos (Soft ODA).

**Conceito**: Tratar sistemas jurídicos como "kernels" e portá-los para diferentes sistemas jurídicos injetando parâmetros culturais.

**Fluxo de Trabalho**:
1. Analisar lei de origem
2. Extrair parâmetros culturais
3. Análise de compatibilidade com sistema jurídico alvo
4. Transformação adaptativa
5. Revisão por especialistas
6. Controle de versão

### 4.6 Camada de Interoperabilidade

#### legalis-interop
Interconversão com múltiplos formatos DSL jurídicos.

**Formatos Suportados**:

| Formato | Origem | Características |
|---------|--------|-----------------|
| Catala | Inria, França | Programação literária |
| Stipula | Univ. Bologna, Itália | Contratos inteligentes |
| L4 | Singapura | Lógica deôntica |
| Akoma Ntoso | OASIS | Documentos legislativos XML |
| LegalRuleML | OASIS | Padrão de regras XML |
| LKIF | ESTRELLA | Intercâmbio de conhecimento jurídico |

### 4.7 Camada de Saída

#### legalis-viz
Motor de visualização.

**Formatos de Saída**:
- Árvores de decisão
- Fluxogramas
- Grafos de dependência
- SVG / PNG / JSON compatível com D3.js

**Temas**: Claro/Escuro

#### legalis-chain
Geração de contratos inteligentes.

**Plataformas Suportadas (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm
- Outros: TON FunC, Algorand Teal, Fuel Sway, Clarity, Noir, Leo, Circom

**Restrição**: Apenas `Deterministic` pode ser convertido (`JudicialDiscretion` não pode ser convertido)

#### legalis-lod
Saída de Dados Abertos Vinculados.

**Ontologias Suportadas**:
- ELI (European Legislation Identifier)
- FaBiO
- LKIF-Core
- Akoma Ntoso
- Dublin Core
- SKOS

**Formatos RDF**: Turtle, N-Triples, RDF/XML, JSON-LD, TriG

### 4.8 Camada de Infraestrutura

#### legalis-audit
Trilhas de auditoria e logging.

**Características**:
- Registro de decisões (contexto completo)
- Integridade de cadeia hash
- Detecção de adulteração
- Conformidade GDPR (suporte aos Artigos 15, 22)

#### legalis-api
Servidor API REST/GraphQL.

**Características**:
- Operações CRUD
- Endpoints de verificação
- Endpoints de simulação
- Autenticação OAuth 2.0
- Multi-tenant
- Limitação de taxa

#### legalis-cli
Interface de linha de comando.

**Comandos**: parse, verify, simulate, visualize, export

**Formatos de Saída**: Text, JSON, YAML, TOML, Table, CSV, HTML

---

## 5. Tecnologias Centrais

### 5.1 DSL Jurídica (Sintaxe, Semântica, Implementação do Parser)

#### 5.1.1 Design de Sintaxe

A DSL do Legalis expressa a estrutura das leis em uma forma próxima à linguagem natural enquanto permite análise formal.

**Estrutura Básica**:
```
STATUTE <id>: "<título>" {
    [JURISDICTION "<jurisdição>"]
    [VERSION <número>]
    [EFFECTIVE_DATE <data>]
    [EXPIRY_DATE <data>]

    WHEN <condição>
    THEN <efeito>

    [EXCEPTION WHEN <condição>]
    [DISCRETION "<descrição>"]

    [AMENDMENT <statute-id>]
    [SUPERSEDES <statute-id>]
}
```

**Expressões de Condição**:
```
<condição> ::= <condição-simples>
             | <condição> AND <condição>
             | <condição> OR <condição>
             | NOT <condição>
             | (<condição>)

<condição-simples> ::= AGE <op> <valor>
                     | INCOME <op> <valor>
                     | HAS <atributo>
                     | DATE <op> <data>
                     | GEOGRAPHIC <tipo-região> <valor>
```

#### 5.1.2 Implementação do Parser

O parser é implementado usando descida recursiva e processa nas seguintes fases:

1. **Análise léxica**: Decomposição em sequência de tokens
2. **Análise sintática**: Construção de AST
3. **Análise semântica**: Verificação de tipos, resolução de referências
4. **Otimização**: Simplificação de expressões de condição

### 5.2 Tipo LegalResult<T> e Valores de Verdade Parciais

#### 5.2.1 Lógica Trivalente

`LegalResult<T>` classifica o resultado do julgamento jurídico em três categorias:

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
```

#### 5.2.2 Valores de Verdade Parciais

A avaliação de condições usa a lógica de 4 valores `PartialBool`:

```rust
pub enum PartialBool {
    True,
    False,
    Unknown,      // Informação insuficiente
    Contradiction, // Contradição
}
```

**Definições de Operações Lógicas**:

| AND | True | False | Unknown | Contradiction |
|-----|------|-------|---------|---------------|
| True | True | False | Unknown | Contradiction |
| False | False | False | False | False |
| Unknown | Unknown | False | Unknown | Contradiction |
| Contradiction | Contradiction | False | Contradiction | Contradiction |

### 5.3 Verificação Formal com Solucionador SMT Z3

#### 5.3.1 Alvos de Verificação

1. **Referências circulares**: Requisitos da Lei A dependem da Lei B, e requisitos de B dependem de A
2. **Leis inalcançáveis**: Condições nunca se tornam True independentemente da entrada
3. **Contradições lógicas**: Efeitos contraditórios sob as mesmas condições
4. **Conflitos constitucionais**: Contradições lógicas com normas superiores

#### 5.3.2 Conversão SMT

Expressões de condição legal são convertidas para o formato SMT-LIB:

```smt2
(declare-const age Int)
(declare-const income Int)
(declare-const has_citizen Bool)

(assert (and (>= age 18) has_citizen))
(assert (not (< income 0)))

(check-sat)
```

### 5.4 Motor de Simulação Estilo ECS

#### 5.4.1 Arquitetura

O motor de simulação adota o padrão Entity-Component-System (ECS):

- **Entity**: Agentes cidadãos
- **Component**: Atributos (idade, renda, residência, etc.)
- **System**: Lógica de aplicação de lei

#### 5.4.2 Execução Paralela

O runtime Tokio e o scheduler work-stealing permitem processamento paralelo de milhões de agentes:

```rust
pub async fn run_simulation(&self) -> SimulationMetrics {
    let (tx, mut rx) = mpsc::channel(1000);

    for agent in &self.population {
        let agent_ref = agent.clone();
        let statutes_ref = self.statutes.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            for statute in statutes_ref {
                let result = Self::apply_law(&agent_ref, &statute);
                let _ = tx_clone.send(result).await;
            }
        });
    }

    // Agregar resultados
    self.aggregate_results(&mut rx).await
}
```

### 5.5 Aceleração GPU (CUDA/OpenCL/WebGPU)

Aceleração GPU é opcionalmente suportada para simulações em grande escala:

- **CUDA**: Para GPUs NVIDIA
- **OpenCL**: Cross-platform
- **WebGPU**: Para browser/WASM

### 5.6 Geração de Contratos Inteligentes (25+ Plataformas)

#### 5.6.1 Fluxo de Geração

1. Extrair partes `Deterministic` da lei
2. Converter para IR da plataforma alvo
3. Gerar código específico da plataforma
4. Verificação formal (opcional)

#### 5.6.2 Exemplo de Saída Solidity

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract DireitosDeVotoAdulto {
    struct Cidadao {
        uint256 idade;
        bool temCidadania;
    }

    function eElegivel(Cidadao memory cidadao)
        public pure returns (bool)
    {
        return cidadao.idade >= 18 && cidadao.temCidadania;
    }
}
```

### 5.7 Dados Abertos Vinculados (RDF/TTL, Múltiplas Ontologias)

#### 5.7.1 Mapeamento de Ontologia

Mapear conceitos jurídicos para ontologias padrão:

```turtle
@prefix eli: <http://data.europa.eu/eli/ontology#> .
@prefix legalis: <http://legalis.rs/ontology#> .

<http://legalis.rs/statute/adult-voting>
    a eli:LegalResource ;
    eli:has_part_with_condition [
        legalis:condition "age >= 18" ;
        legalis:effect "voting_rights"
    ] .
```

---

## 6. Implementações Jurisdicionais

### 6.1 Sistema Jurídico Japonês (Constituição, Código Civil, Previdência)

#### 6.1.1 Constituição do Japão

O crate legalis-jp fornece uma representação estruturada da Constituição do Japão:

**Estrutura de Capítulos**:
- Capítulo I: O Imperador
- Capítulo II: Renúncia à Guerra
- Capítulo III: Direitos e Deveres do Povo
- ...
- Capítulo XI: Disposições Suplementares

**Representação DSL de Disposições Principais**:
```
STATUTE jp-constitution-art25: "Direito à Vida" {
    JURISDICTION "JP"
    REFERENCE "Constituição do Japão, Artigo 25"

    DISCRETION "O padrão específico de 'padrões mínimos de vida
                saudável e cultural' será determinado por legislação
                considerando consenso social e condições fiscais"
}
```

#### 6.1.2 Artigo 709 do Código Civil (Responsabilidade Civil)

```
STATUTE minpo-709: "Danos por Ato Ilícito" {
    JURISDICTION "JP"
    REFERENCE "Código Civil Artigo 709"

    WHEN HAS ato_intencional OR HAS negligencia
    AND HAS violacao_de_direitos
    AND HAS nexo_causal
    AND HAS danos

    THEN OBLIGATION "Compensação por danos"

    DISCRETION "Determinação de negligência, julgamento de causalidade,
                e cálculo de danos são de discricionariedade do tribunal"
}
```

#### 6.1.3 Sistema de Previdência

Sistema de determinação de elegibilidade para benefícios previdenciários:

```
STATUTE welfare-basic: "Assistência Previdenciária Básica" {
    JURISDICTION "JP"

    WHEN INCOME <= 30000
    THEN GRANT "Assistência previdenciária básica"
}

STATUTE welfare-senior: "Suplemento de Pensão para Idosos" {
    JURISDICTION "JP"

    WHEN AGE >= 65 AND INCOME <= 50000
    THEN GRANT "Suplemento de pensão para idosos"
}
```

### 6.2 Alemanha, França, EUA (Planejado)

Implementações para cada jurisdição estão planejadas:

| Jurisdição | Status | Áreas de Foco |
|------------|--------|---------------|
| Alemanha (DE) | Em desenvolvimento | BGB (Código Civil), GG (Lei Fundamental) |
| França (FR) | Em desenvolvimento | Code civil, Constituição |
| EUA (US) | Em desenvolvimento | UCC, Constituição, Jurisprudência |

### 6.3 Adaptação de Parâmetros Culturais (Soft ODA)

Os seguintes parâmetros culturais são considerados na portabilidade internacional de sistemas jurídicos:

1. **Sistema jurídico**: Civil law vs Common law vs Direito religioso
2. **Estrutura linguística**: Traduzibilidade de termos jurídicos
3. **Normas sociais**: Tabus, costumes, restrições religiosas
4. **Estrutura administrativa**: Centralizado vs Federal
5. **Sistema judicial**: Júri vs Juízes profissionais

---

## 7. Estudos de Caso

### 7.1 Sistema de Determinação de Elegibilidade para Benefícios Previdenciários

#### 7.1.1 Visão Geral do Sistema

Determinação automática de elegibilidade para 6 programas previdenciários:

1. Assistência previdenciária básica
2. Suplemento de pensão para idosos
3. Benefício de apoio à criança
4. Assistência para deficientes
5. Assistência habitacional de emergência
6. Suplemento de saúde

#### 7.1.2 Fluxo de Trabalho da Demo

```
Etapa 1: Parse DSL (7 leis)
Etapa 2: Verificação de leis
Etapa 3: Criação de dados de cidadãos
Etapa 4: Avaliação de elegibilidade e registro de auditoria
Etapa 5: Visualização de árvore de decisão
Etapa 6: Simulação de população (500 cidadãos)
Etapa 7: Verificação de integridade da trilha de auditoria
```

#### 7.1.3 Resultados

- **Decisões determinísticas**: 85% dos casos
- **JudicialDiscretion**: 15% dos casos (julgamentos sobre "urgência", "necessidade genuína", etc.)

### 7.2 Simulação do Artigo 709 do Código Civil (Responsabilidade Civil)

#### 7.2.1 Cenários de Teste

Simulação de 5 cenários:

1. **Ato ilícito intencional claro** → `Deterministic(Liable)`
2. **Ato ilícito por negligência** → `Deterministic(Liable)`
3. **Caso limítrofe** → `JudicialDiscretion`
4. **Sem ato ilícito** → `Deterministic(NotLiable)`
5. **Sem nexo causal** → `Deterministic(NotLiable)`

#### 7.2.2 Resultados da Simulação

```
Agente 1: Deterministic(Responsável por danos)
Agente 2: Deterministic(Responsável por danos)
Agente 3: JudicialDiscretion(Julgamento de causalidade é de discricionariedade do tribunal)
Agente 4: Deterministic(Não responsável)
Agente 5: Deterministic(Não responsável)
```

### 7.3 Análise Comparativa de Direito de Responsabilidade Civil em 4 Países

#### 7.3.1 Espectro de Filosofia Jurídica

| País | Código | Características |
|------|--------|-----------------|
| Japão | Código Civil Art. 709 | Cláusula geral (ampla discricionariedade) |
| Alemanha | BGB §823/§826 | Interesses protegidos enumerados |
| França | Code civil Art. 1240 | Máxima abstração |
| EUA | Jurisprudência | Tipificado (Battery, etc.) |

#### 7.3.2 Avaliação do Mesmo Caso

Avaliação do mesmo caso de responsabilidade civil sob os sistemas jurídicos de 4 países:

```
Japão: JudicialDiscretion (ampla discricionariedade)
Alemanha: Deterministic (corresponde ao tipo enumerado)
França: JudicialDiscretion (disposições abstratas)
EUA: Deterministic (Battery aplicável)
```

### 7.4 Visualização da Estrutura da Constituição do Japão

Visualização de estrutura em 3 camadas:

```
Constituição do Japão
├── Capítulo I: O Imperador
│   ├── Artigo 1: Status do Imperador
│   ├── Artigo 2: Sucessão ao Trono
│   └── ...
├── Capítulo II: Renúncia à Guerra
│   └── Artigo 9: Renúncia à Guerra
├── Capítulo III: Direitos e Deveres do Povo
│   ├── Artigo 11: Direitos Humanos Fundamentais
│   ├── Artigo 13: Respeito aos Indivíduos
│   ├── Artigo 14: Igualdade Perante a Lei
│   └── ...
└── ...
```

---

## 8. Referência de API e Detalhes Técnicos

### 8.1 Tipos e Traits Principais

#### 8.1.1 legalis-core

```rust
// Tipo de lógica trivalente
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

// Trait de entidade legal
pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}

// Trait de contexto de avaliação
pub trait EvaluationContext: Send + Sync {
    fn get_attribute(&self, entity_id: &str, name: &str) -> Option<Value>;
    fn set_attribute(&mut self, entity_id: &str, name: String, value: Value) -> Result<()>;
}

// Lei
pub struct Statute {
    pub id: String,
    pub title: String,
    pub primary_effect: Effect,
    pub preconditions: Vec<Condition>,
    pub jurisdiction: String,
    pub temporal_validity: TemporalValidity,
}

// Condição
pub enum Condition {
    Age { operator: ComparisonOp, value: u32 },
    Income { operator: ComparisonOp, value: i64 },
    HasAttribute(String),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

// Efeito
pub enum EffectType {
    Grant,
    Revoke,
    Obligation,
    Prohibition,
    Discretion,
}
```

### 8.2 Endpoints API REST / GraphQL

#### 8.2.1 API REST

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | /api/v1/statutes | Obter lista de leis |
| GET | /api/v1/statutes/{id} | Obter detalhes da lei |
| POST | /api/v1/statutes | Criar lei |
| PUT | /api/v1/statutes/{id} | Atualizar lei |
| DELETE | /api/v1/statutes/{id} | Excluir lei |
| POST | /api/v1/verify | Executar verificação |
| POST | /api/v1/simulate | Executar simulação |
| POST | /api/v1/evaluate | Executar avaliação de elegibilidade |

#### 8.2.2 GraphQL

```graphql
type Query {
    statute(id: ID!): Statute
    statutes(jurisdiction: String, limit: Int): [Statute!]!
    verify(statuteIds: [ID!]!): VerificationResult!
}

type Mutation {
    createStatute(input: StatuteInput!): Statute!
    updateStatute(id: ID!, input: StatuteInput!): Statute!
    deleteStatute(id: ID!): Boolean!
}

type Statute {
    id: ID!
    title: String!
    jurisdiction: String!
    conditions: [Condition!]!
    effect: Effect!
}
```

### 8.3 Sistema de Comandos CLI

```bash
# Parse
legalis parse <arquivo.dsl> [--format json|yaml]

# Verificar
legalis verify <arquivo.dsl> [--strict]

# Simular
legalis simulate <arquivo.dsl> --population 1000

# Visualizar
legalis visualize <arquivo.dsl> --output tree.svg

# Exportar
legalis export <arquivo.dsl> --format solidity|catala|l4|rdf
```

### 8.4 Formatos de Saída

| Formato | Uso |
|---------|-----|
| JSON | Respostas de API, troca de dados |
| YAML | Arquivos de configuração, legível por humanos |
| CSV | Dados tabulares |
| HTML | Relatórios |
| SVG | Visualização |
| RDF/TTL | Web semântica |
| Solidity | Contratos inteligentes |

---

## 9. Avaliação

### 9.1 Benchmarks de Performance

| Operação | Alvo | Tempo |
|----------|------|-------|
| Parse DSL | 100 leis | 15ms |
| Verificação | 100 leis | 250ms |
| Simulação | 10.000 agentes | 1.2s |
| Simulação | 100.000 agentes | 8.5s |
| Geração de contrato inteligente | 1 lei | 45ms |
| Exportação RDF | 100 leis | 120ms |

### 9.2 Qualidade do Código

- **Cobertura de testes**: Testes de integração, testes de propriedade, testes de snapshot
- **Análise estática**: Clippy (política zero warnings)
- **Documentação**: rustdoc para todas as APIs públicas

### 9.3 Avaliação de Usabilidade

- **CLI**: Sistema de comandos intuitivo
- **API**: Design RESTful, suporte GraphQL
- **Mensagens de erro**: Com sugestões de correção
- **Documentação**: Suporte em Japonês/Inglês

---

## 10. Trabalhos Futuros

### 10.1 Frontend Web UI

- Dashboard baseado em React
- Visualização de simulação em tempo real
- Recursos de edição colaborativa

### 10.2 Extensão VS Code

- Destaque de sintaxe DSL
- Verificação em tempo real
- Autocompletar

### 10.3 Integração Jupyter Notebook

- Bindings Python via PyO3
- Análise interativa
- Widgets de visualização

### 10.4 Jurisdições Adicionais

- Direito da UE (integração EURLex)
- Direito internacional (tratados, acordos)
- Direito religioso (jurisprudência islâmica)

---

## 11. Conclusão

O Legalis-RS apresenta uma nova abordagem para codificar a lei tornando o "limite entre computabilidade e julgamento humano" explícito no sistema de tipos.

**Principais Conquistas**:

1. **Fundamento filosófico**: "Governança como Código, Justiça como Narrativa"
2. **Sistema de tipos**: Lógica trivalente via `LegalResult<T>`
3. **Arquitetura integrada**: Design abrangente com 7 camadas e 16 crates
4. **Implementação**: Aproximadamente 450.000 linhas de código Rust
5. **Verificação**: Integração com solucionador SMT Z3
6. **Simulação**: Motor estilo ECS (suporte a aceleração GPU)
7. **Saída**: 25+ blockchains, RDF/TTL, múltiplos formatos

**Filosofia Central**: *"Nem tudo deve ser computável."*

Não a automação completa da lei, mas a clara separação de domínios que devem ser automatizados dos domínios que requerem julgamento humano. Esta é a arquitetura de "jurisprudência generativa" que o Legalis-RS almeja.

---

## Referências

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. Governatori, G., & Shams, Z. (2019). L4: Legal Language and Logic for Law. *JURIX 2019*.
5. Azzopardi, S., & Pace, G. J. (2018). Stipula: A domain-specific language for legal contracts. *JURIX 2018*.
6. Palmirani, M., & Vitali, F. (2011). Akoma-Ntoso for Legal Documents. *Legislative XML for the Semantic Web*.
7. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

## Apêndice

### A. Especificação da Gramática DSL

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
condition    = simple_cond | compound_cond ;
compound_cond = condition ("AND" | "OR") condition | "NOT" condition | "(" condition ")" ;
simple_cond  = age_cond | income_cond | has_cond | date_cond | geographic_cond ;
age_cond     = "AGE" comparison_op number ;
income_cond  = "INCOME" comparison_op number ;
has_cond     = "HAS" identifier ;
effect       = "GRANT" string | "REVOKE" string | "OBLIGATION" string | "PROHIBITION" string ;
```

### B. Lista de Definições de Tipos

Para definições completas dos tipos principais, veja `crates/legalis-core/src/lib.rs`.

### C. Opções de Configuração

```toml
[legalis]
default_jurisdiction = "JP"
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

*"Code is Law", dizem, mas nós adotamos a abordagem de "Lei se torna Código". No entanto, incorporamos um tipo chamado 'Humanidade' nesse código.*

---

**Equipe de Desenvolvimento Legalis-RS**
Versão 0.2.0 | 2024
