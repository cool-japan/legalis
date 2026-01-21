# Legalis-BR: Brazilian Jurisdiction Support / Suporte à Jurisdição Brasileira

**Legalis-RS Brazilian Jurisdiction Module** / **Módulo de Jurisdição Brasileira do Legalis-RS**

[![Crates.io](https://img.shields.io/crates/v/legalis-br.svg)](https://crates.io/crates/legalis-br)
[![Documentation](https://docs.rs/legalis-br/badge.svg)](https://docs.rs/legalis-br)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## English

### Overview

This crate provides comprehensive modeling of Brazilian law within the Legalis-RS legal framework. It implements Brazil's civil law system based on the Romano-Germanic tradition with Portuguese legal terminology.

### Legal System

Brazil follows a **civil law system** (sistema de direito civil) with:
- **Written codes** as primary sources of law
- **Portuguese language** as the official legal language
- **Federal structure** with Union, States (26 + Federal District), and Municipalities
- **Consumer protection** as a fundamental right
- **Social function** principles in contracts and property

### Implemented Legal Domains

#### 1. Constitutional Law (Constituição Federal 1988)
- Fundamental rights (Art. 5 with 78 clauses)
- Social rights (Arts. 6-11: education, health, work)
- Federal structure (Union, States, Municipalities)
- Democratic principles and checks and balances

#### 2. Consumer Protection Law (CDC - Lei 8.078/1990)
- Consumer as vulnerable party principle (princípio da vulnerabilidade)
- Strict liability for defective products/services (Arts. 12-14)
- 7-day withdrawal right (direito de arrependimento, Art. 49)
- 16 types of abusive clauses (Art. 51)
- PROCON enforcement system

#### 3. Labor Law (CLT - Consolidação das Leis do Trabalho)
- Maximum 44 hours per week
- 13th salary (Christmas bonus - décimo terceiro salário)
- Vacation + 1/3 bonus (férias + um terço)
- FGTS (8% monthly severance fund - Fundo de Garantia)
- Labor Justice system (Justiça do Trabalho)

#### 4. Civil Code (Código Civil 2002)
- Social function of contract (função social do contrato)
- Objective good faith (boa-fé objetiva)
- General Provisions, Obligations, Property
- Persons (natural and legal), Family Law, Succession

#### 5. Tax Law (Sistema Tributário Nacional)
- ICMS (state VAT on goods/services)
- ISS (municipal service tax)
- PIS/COFINS (social contributions)
- Federal, state, and municipal tax division

#### 6. Corporate Law (Lei das S.A. 6.404/1976)
- Novo Mercado governance levels
- CVM (Securities Commission) regulation
- Tag-along rights in acquisitions
- Corporate governance requirements

#### 7. Data Protection (LGPD - Lei 13.709/2018)
- Brazil's GDPR: 10 legal bases, 9 rights
- ANPD (National Data Protection Authority)
- Personal and sensitive data protection
- Cross-border data transfer rules

### Citation Format

Brazilian legal citations follow the format:
```
Lei nº [number]/[year], Art. [article]º, §[paragraph]º, inciso [clause]
```

Examples:
- `Lei nº 8.078/1990, Art. 5º` (CDC Article 5)
- `Lei nº 5.452/1943, Art. 58, §1º` (CLT Article 58, Paragraph 1)
- `Constituição Federal, Art. 5º, inciso X` (CF/88 Art. 5, Clause X)

### Features

- **Bilingual Support**: Portuguese (primary/authoritative) + English translations
- **Comprehensive Types**: All major legal entities and concepts
- **Validation Functions**: Type-safe legal validation
- **Citation Support**: Brazilian legal citation formatting
- **Test Coverage**: Extensive test suite

### Usage Example

```rust
use legalis_br::consumer_protection::*;
use legalis_br::citation::format_cdc_citation;

// Consumer protection example
let consumer_right = ConsumerRight {
    nome_pt: "Direito de arrependimento".to_string(),
    name_en: "Right of withdrawal".to_string(),
    artigo: 49,
    prazo_dias: 7,
};

// Format citation
let citation = format_cdc_citation(49, None, None);
// Result: "Lei nº 8.078/1990, Art. 49"

// Validate withdrawal period
match validate_withdrawal_right(&consumer_right) {
    Ok(_) => println!("Valid withdrawal right"),
    Err(e) => println!("Invalid: {}", e),
}
```

### Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
legalis-br = "0.1.3"
```

---

## Português

### Visão Geral

Este crate fornece modelagem abrangente do direito brasileiro dentro do framework legal Legalis-RS. Implementa o sistema de direito civil brasileiro baseado na tradição romano-germânica com terminologia legal em português.

### Sistema Jurídico

O Brasil segue um **sistema de direito civil** (civil law) com:
- **Códigos escritos** como fontes primárias do direito
- **Língua portuguesa** como idioma jurídico oficial
- **Estrutura federal** com União, Estados (26 + Distrito Federal) e Municípios
- **Proteção do consumidor** como direito fundamental
- **Função social** nos contratos e propriedade

### Domínios Legais Implementados

#### 1. Direito Constitucional (Constituição Federal de 1988)
- Direitos fundamentais (Art. 5º com 78 incisos)
- Direitos sociais (Arts. 6º-11: educação, saúde, trabalho)
- Estrutura federal (União, Estados, Municípios)
- Princípios democráticos e checks and balances

#### 2. Direito do Consumidor (CDC - Lei 8.078/1990)
- Princípio da vulnerabilidade do consumidor
- Responsabilidade objetiva por produtos/serviços defeituosos (Arts. 12-14)
- Direito de arrependimento de 7 dias (Art. 49)
- 16 tipos de cláusulas abusivas (Art. 51)
- Sistema de fiscalização PROCON

#### 3. Direito do Trabalho (CLT - Consolidação das Leis do Trabalho)
- Jornada máxima de 44 horas semanais
- Décimo terceiro salário
- Férias + um terço
- FGTS (8% mensal - Fundo de Garantia do Tempo de Serviço)
- Justiça do Trabalho

#### 4. Código Civil (Código Civil de 2002)
- Função social do contrato
- Boa-fé objetiva
- Parte Geral, Obrigações, Direito das Coisas
- Pessoas (naturais e jurídicas), Família, Sucessões

#### 5. Direito Tributário (Sistema Tributário Nacional)
- ICMS (imposto estadual sobre circulação de mercadorias e serviços)
- ISS (imposto municipal sobre serviços)
- PIS/COFINS (contribuições sociais)
- Divisão de competências tributárias (federal, estadual, municipal)

#### 6. Direito Societário (Lei das S.A. 6.404/1976)
- Níveis de governança do Novo Mercado
- Regulação da CVM (Comissão de Valores Mobiliários)
- Direito de tag-along em aquisições
- Requisitos de governança corporativa

#### 7. Proteção de Dados (LGPD - Lei 13.709/2018)
- GDPR brasileiro: 10 bases legais, 9 direitos
- ANPD (Autoridade Nacional de Proteção de Dados)
- Proteção de dados pessoais e sensíveis
- Regras de transferência internacional de dados

### Formato de Citação

Citações legais brasileiras seguem o formato:
```
Lei nº [número]/[ano], Art. [artigo]º, §[parágrafo]º, inciso [inciso]
```

Exemplos:
- `Lei nº 8.078/1990, Art. 5º` (CDC Artigo 5)
- `Lei nº 5.452/1943, Art. 58, §1º` (CLT Artigo 58, Parágrafo 1)
- `Constituição Federal, Art. 5º, inciso X` (CF/88 Art. 5, Inciso X)

### Recursos

- **Suporte Bilíngue**: Português (primário/autoritativo) + traduções em inglês
- **Tipos Abrangentes**: Todas as principais entidades e conceitos jurídicos
- **Funções de Validação**: Validação legal type-safe
- **Suporte a Citações**: Formatação de citações legais brasileiras
- **Cobertura de Testes**: Conjunto extensivo de testes

### Exemplo de Uso

```rust
use legalis_br::consumer_protection::*;
use legalis_br::citation::format_cdc_citation;

// Exemplo de proteção do consumidor
let direito_consumidor = ConsumerRight {
    nome_pt: "Direito de arrependimento".to_string(),
    name_en: "Right of withdrawal".to_string(),
    artigo: 49,
    prazo_dias: 7,
};

// Formatar citação
let citacao = format_cdc_citation(49, None, None);
// Resultado: "Lei nº 8.078/1990, Art. 49"

// Validar prazo de arrependimento
match validate_withdrawal_right(&direito_consumidor) {
    Ok(_) => println!("Direito de arrependimento válido"),
    Err(e) => println!("Inválido: {}", e),
}
```

### Instalação

Adicione ao seu `Cargo.toml`:
```toml
[dependencies]
legalis-br = "0.1.3"
```

## License / Licença

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../../LICENSE-MIT))

at your option.

## Contribution / Contribuição

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Copyright

Copyright © 2025 COOLJAPAN OU (Team Kitasan)
