# Legalis-RS: Generatiivse Õigusteaduse Arhitektuur

## Õiguse ja Narratiivi Eraldamine: Plaan "Valitsemiseks Koodina"

---

**Autorid**: Legalis-RS Arendusmeeskond
**Versioon**: 0.2.0
**Keel**: Rust (Edition 2024)
**Litsents**: MIT / Apache 2.0

---

## Kokkuvõte

See artikkel tutvustab **Legalis-RS**-i, Rust raamistikku loomukeelsete õigusdokumentide rangeks eraldamiseks ja struktureerimiseks **deterministlikuks loogikaks (Code)** ja **kohtulikuks diskretsiooniks (Narrative)**.

Kaasaegsed õigussüsteemid sisaldavad arvuti automatiseerimisele alluvate valdkondade (vanuse nõuded, sissetuleku piirmäärad, tähtaegade arvutused) ja inimese tõlgendamist ja otsustust nõudvate valdkondade ("õiglane põhjus", "avalik moraal") segu. Varasemad lähenemised on jätnud selle piiri ebamääraseks või püüdnud liigset automatiseerimist, mis püüdis muuta kõik arvutatavaks.

Legalis-RS tutvustab kolmeväärtuselist loogikatüüpi `LegalResult<T>`, kasutades Rusti tüübisüsteemi, et muuta see piir tüübi tasandil selgesõnaliseks. See võimaldab uut paradigmat õigusliku silumise, simulatsiooni ja rahvusvahelise portimise jaoks, vältides samal ajal "algoritmilist autokraatiat" AI ajastul.

**Peamised Tehnilised Panused**:
1. Õiguslik domeenispetsiifiline keel (DSL) ja parseri implementatsioon
2. Formaalne verifitseerimine Z3 SMT lahendajaga
3. ECS-stiilis simulatsioonimootor sotsiaalse mõju ennustamiseks
4. Nutikate lepingute genereerimine 25+ plokiahela platvormile
5. Linked Open Data (RDF/TTL) integratsioon semantilise veebi jaoks
6. Õigussüsteemi implementatsioonid 4 riigile kultuuriliste parameetrite kohandamisega (Soft ODA)

**Põhifilosoofia**: *"Mitte kõik ei peaks olema arvutatav."*

---

## 1. Sissejuhatus

### 1.1 Taust: Õiguse ja Arvutuse Vaheline Suhe

Lawrence Lessigi kuulus tees "Code is Law" osutas, et küberruumi arhitektuuril (koodil) on õigusega võrdväärne regulatiivne jõud. Siiski, Legalis-RS pöörab selle ümber, võttes kasutusele lähenemise "**Õigus muutub Koodiks**".

Õiguse kodifitseerimine pakub järgmisi eeliseid:

- **Verifitseeritavus**: Loogiliste vastuolude tuvastamine kompileerimise ajal
- **Simulatsioon**: Sotsiaalsete mõjude ennustamine enne jõustamist
- **Koostalitlusvõime**: Erinevate õigussüsteemide vahel teisendamine ja võrdlemine
- **Läbipaistvus**: Õiguslike otsustusprotsesside täielikud auditi jäljed

Siiski on kõigi seaduste arvutatavaks muutmine filosoofiliselt ja praktiliselt ohtlik. Õigus sisaldab olemuslikult valdkondi, mis nõuavad "inimlikku otsustust", ja seda eirav automatiseerimine võib viia "AI autokraatiani".

### 1.2 Probleemi Püstitus: Õigusliku Töötlemise Väljakutsed AI Ajastul

Kaasaegne õigustehnoloogia (LegalTech) seisab silmitsi mitme põhilise väljakutsega:

1. **Ebamäärasuse käsitlemine**: Paljud õiguslikud terminid on tahtlikult ebamäärased, eeldades juhtumipõhist tõlgendamist
2. **Kontekstisõltuvus**: Sama säte võib sõltuvalt sotsiaalsest ja kultuurilisest kontekstist erinevalt tõlgendatud olla
3. **Ajaline muutus**: Seadusi muudetakse ja tunnistatakse kehtetuks, nõudes järjepidevuse haldamist ajas
4. **Rahvusvahelised erinevused**: Iga riigi õigussüsteemid erinevad alates filosoofilistest alustest

Olemasolevad õiguslikud DSL-id (Catala, L4, Stipula) on käsitlenud mõnda neist väljakutsetest, kuid ükski pole võtnud lähenemist, mis muudaks "arvutatavuse ja inimlikku otsustuse vahelise piiri" tüübisüsteemis selgesõnaliseks.

### 1.3 Ettepanek: Arvutatavuse ja Kohtuliku Diskretsiooni Eraldamine

Legalis-RS-i tuum on kolmeväärtuselist loogika tutvustamine `LegalResult<T>` tüübi kaudu:

```rust
pub enum LegalResult<T> {
    /// [Deterministlik Valdkond] Automaatselt töödeldavad õiguslikud tulemused
    Deterministic(T),

    /// [Diskretsioonivaldkond] Valdkond, mis nõuab inimlikku otsustust
    JudicialDiscretion {
        issue: String,           // Käsitletav küsimus
        context_id: Uuid,        // Kontekstuaalsed andmed
        narrative_hint: Option<String>, // LLM-i viitearvamus
    },

    /// [Loogiline Kokkuvarisemine] Viga seaduses endas
    Void { reason: String },
}
```

See tüüp tagab, et õigusliku töötlemise tulemus on alati klassifitseeritud ühte kolmest kategooriast. Süsteem lõpetab töötlemise `JudicialDiscretion`-i jõudmisel ja delegeerib otsustuse inimestele. See muutub "tüübitaseme kindluseks" AI autokraatia vastu.

---

## 2. Seotud Töö

### 2.1 Arvutusliku Õiguse Ajalugu

Õiguse ja arvutite vaheline suhe ulatub 1950ndate LARC (Legal Analysis and Research Computer) projektini.

| Ajastu | Tehnoloogia | Omadused |
|--------|-------------|----------|
| 1950ndad | LARC | Esimene õigusliku teabe otsimise süsteem |
| 1970ndad | MYCIN-tüüpi ekspertsüsteemid | Reeglipõhine arutlus |
| 1980ndad | HYPO | Juhtumipõhine arutlus |
| 1990ndad | XML/SGML standardiseerimine | Õigusdokumentide struktureerimine |
| 2000ndad | Semantiline veeb | Ontoloogiapõhine õiguslike teadmiste esitus |
| 2010ndad | Masinõpe | Õiguslikud ennustusmudelid |
| 2020ndad | LLM + Formaalne verifitseerimine | Hübriidne lähenemine |

### 2.2 Selle Projekti Positsioon

Legalis-RS laiendab olemasolevat uurimistööd järgmistel viisidel:

1. **Tüübitaseme diskretsiooni märgistamine**: Kolmeväärtuseline loogika `LegalResult<T>` kaudu
2. **Integreeritud arhitektuur**: Parse→Verify→Simulate→Output torujuhe
3. **Mitmevorminguline koostalitlusvõime**: Teisendamine Catala/L4/Stipula/Akoma Ntoso vahel
4. **Rahvusvahelistumise disain**: Kultuuriliste parameetrite kohandamine (Soft ODA)
5. **Plokiahela integratsioon**: Nutikate lepingute genereerimine 25+ platvormile

---

## 3. Filosoofia & Disainipõhimõtted

### 3.1 "Valitsemine Koodina, Õiglus Narratiivina"

Legalis-RS-i loosung peegeldab olulist erinevust valitsemise ja õigluse vahel:

- **Valitsemine**: Reeglite rakendamine, protseduuriline vastavus, sobivuse määramine → **Kodifitseeritav**
- **Õiglus**: Võrdsuse realiseerimine, kontekstuaalne tõlgendamine, väärtusotsustus → **Räägitud narratiivina**

### 3.2 Kolmeväärtuseline Loogika Disain

`LegalResult<T>` kolm väärtust vastavad järgmistele õigusfilosoofilistele kontseptsioonidele:

| Tüüp | Õigusfilosoofiline Kontseptsioon | Töötleja Agent |
|------|----------------------------------|----------------|
| `Deterministic(T)` | Mehaaniliselt rakendatavad reeglid | Arvuti |
| `JudicialDiscretion` | Tõlgendamist nõudvad põhimõtted | Inimene |
| `Void` | Õiguslikud lüngad/vastuolud | Seadusandja (vajab parandust) |

### 3.3 "Mitte kõik ei peaks olema arvutatav"

Kiusatuse vastu muuta kõik arvutatavaks ütleb Legalis-RS selgelt "Ei". Järgmised valdkonnad on tahtlikult disainitud mittearvutatavaks:

1. **Õiglane põhjus**
2. **Avalik kord ja moraal**
3. **Heas usus**
4. **Mõistlikkus**

### 3.4 AI Autokraatia Vältimine

Legalis-RS väldib AI autokraatiat järgmiste mehhanismide kaudu:

1. **Tüübipõhine sundpeatumine**: Automaatne peatumine `JudicialDiscretion`-i jõudmisel
2. **Kohustuslikud auditi jäljed**: Kõigi otsustusprotsesside salvestamine
3. **Seletatavus**: Otsustuse põhjenduste struktureeritud väljund
4. **Garanteeritud inimsilmus**: Inimesed teevad diskretsioonivaldkondades alati lõplikud otsused

---

## 4. Süsteemi Arhitektuur

### 4.1 7-Kihiline Arhitektuuri Ülevaade

Legalis-RS koosneb järgmistest 7 kihist:

```
┌─────────────────────────────────────────────────────────┐
│                   Infrastruktuuri Kiht                   │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                      Väljundi Kiht                       │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│               Koostalitlusvõime Kiht                     │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│               Rahvusvahelistumise Kiht                   │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│            Simulatsiooni & Analüüsi Kiht                 │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                   Intelligentsuse Kiht                   │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                       Tuumik Kiht                        │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Tuumik Kiht

#### legalis-core
Projekti filosoofilise tuumiku implementeeriv crate.

**Peamised Tüübidefinitsioonid**:
- `LegalResult<T>`: Kolmeväärtuseline loogikatüüp
- `Statute`: Seaduste põhiesitus
- `Condition`: Tingimuste avaldised (AND/OR/NOT, vanus, sissetulek jne)
- `Effect`: Õiguslikud mõjud (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Õigusliku domeenispetsiifilise keele parser.

**DSL Süntaksi Näide**:
```
STATUTE adult-voting: "Täiskasvanu Hääletamisõigused" {
    JURISDICTION "EE"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Hääletamisõigused"

    DISCRETION "Vaimse võimekuse määramine nõuab arsti diagnoosi"
}
```

### 4.3 Intelligentsuse Kiht

#### legalis-llm
LLM pakkuja abstraktsioonikiht.

**Toetatud Pakkujad**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- Kohalik LLM

#### legalis-verifier
Formaalse verifitseerimise mootor.

**Verifitseerimise Sihtmärgid**:
- Ringviidete tuvastamine
- Kättesaamatu seaduse (Dead Statute) tuvastamine
- Loogiliste vastuolude tuvastamine
- Põhiseadusliku konflikti kontroll
- Ebamäärasuse analüüs

### 4.4 Simulatsiooni Kiht

#### legalis-sim
ECS-stiilis simulatsioonimootor.

**Funktsioonid**:
- Populatsioonipõhine simulatsioon (toetab miljoneid agente)
- Monte Carlo simulatsioon
- Tundlikkuse analüüs
- A/B testimine
- GPU kiirendus (CUDA/OpenCL/WebGPU)

### 4.5 Rahvusvahelistumise Kiht

#### legalis-i18n
Mitmekeelne ja mitmejurisdiktsiooniline tugi.

**Toetatud Jurisdiktsioonid**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG, EE

### 4.6 Väljundi Kiht

#### legalis-chain
Nutikate lepingute genereerimine.

**Toetatud Platvormid (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm

**Piirang**: Ainult `Deterministic` on teisendatav (`JudicialDiscretion` ei ole teisendatav)

---

## 5. Tuumtehnoloogiad

### 5.1 Õiguslik DSL

**Põhistruktuur**:
```
STATUTE <id>: "<title>" {
    [JURISDICTION "<jurisdiction>"]
    [VERSION <number>]
    [EFFECTIVE_DATE <date>]

    WHEN <condition>
    THEN <effect>

    [EXCEPTION WHEN <condition>]
    [DISCRETION "<description>"]
}
```

### 5.2 LegalResult<T> Tüüp

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

### 5.3 Formaalne Verifitseerimine Z3 SMT Lahendajaga

**Verifitseerimise Sihtmärgid**:
1. Ringviited
2. Kättesaamatud seadused
3. Loogilised vastuolud
4. Põhiseaduslikud konfliktid

### 5.4 ECS-Stiilis Simulatsioonimootor

Simulatsioonimootor kasutab Entity-Component-System (ECS) mustrit:
- **Entity**: Kodaniku agendid
- **Component**: Atribuudid (vanus, sissetulek, elukoht jne)
- **System**: Seaduse rakendamise loogika

---

## 6. Jurisdiktsiooni Implementatsioonid

### 6.1 Jaapani Õigussüsteem

legalis-jp crate pakub Jaapani põhiseaduse struktureeritud esitust.

### 6.2 Saksamaa, Prantsusmaa, USA (Planeeritud)

| Jurisdiktsioon | Staatus | Fookusalad |
|----------------|---------|------------|
| Saksamaa (DE) | Arenduses | BGB, GG |
| Prantsusmaa (FR) | Arenduses | Code civil |
| USA (US) | Arenduses | UCC, Põhiseadus |

---

## 7. Juhtumiuuringud

### 7.1 Heaolu Sobivuse Määramise Süsteem

**Tulemused**:
- **Deterministlikud otsused**: 85% juhtumitest
- **JudicialDiscretion**: 15% juhtumitest

### 7.2 Tsiviilkoodeksi Artikkel 709 (Delikt) Simulatsioon

**Testistsenaariumid**:
1. Selge tahtlik delikt → `Deterministic(Liable)`
2. Hooletusest tingitud delikt → `Deterministic(Liable)`
3. Piirijuhtum → `JudicialDiscretion`

---

## 8. API Spetsifikatsioon & Tehnilised Üksikasjad

### 8.1 Peamised Tüübid

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

pub struct Statute {
    pub id: String,
    pub title: String,
    pub primary_effect: Effect,
    pub preconditions: Vec<Condition>,
    pub jurisdiction: String,
}
```

### 8.2 CLI Käsusüsteem

```bash
legalis parse <file.dsl> [--format json|yaml]
legalis verify <file.dsl> [--strict]
legalis simulate <file.dsl> --population 1000
legalis visualize <file.dsl> --output tree.svg
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Hindamine

### 9.1 Jõudluse Võrdlusnäitajad

| Toiming | Sihtmärk | Aeg |
|---------|----------|-----|
| DSL parsimine | 100 seadust | 15ms |
| Verifitseerimine | 100 seadust | 250ms |
| Simulatsioon | 10 000 agenti | 1.2s |
| Simulatsioon | 100 000 agenti | 8.5s |

---

## 10. Tulevane Töö

- Veebi UI Frontend (React-põhine)
- VS Code laiendus
- Jupyter Notebook integratsioon
- Täiendavad jurisdiktsioonid

---

## 11. Järeldus

Legalis-RS esitab uue lähenemise õiguse kodifitseerimiseks, muutes "arvutatavuse ja inimlikku otsustuse vahelise piiri" tüübisüsteemis selgesõnaliseks.

**Peamised Saavutused**:

1. **Filosoofiline alus**: "Valitsemine Koodina, Õiglus Narratiivina"
2. **Tüübisüsteem**: Kolmeväärtuseline loogika `LegalResult<T>` kaudu
3. **Integreeritud arhitektuur**: Põhjalik disain 7 kihi ja 16 crate'iga
4. **Implementatsioon**: Umbes 450 000 rida Rust koodi
5. **Verifitseerimine**: Z3 SMT lahendaja integratsioon
6. **Simulatsioon**: ECS-stiilis mootor (GPU kiirenduse tugi)
7. **Väljund**: 25+ plokiahelat, RDF/TTL, mitu vormingut

**Põhifilosoofia**: *"Mitte kõik ei peaks olema arvutatav."*

---

## Viited

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

*"Code is Law," öeldakse, kuid meie võtame lähenemise "Õigus muutub Koodiks". Siiski, me põimime sellesse koodi tüübi nimega 'Inimlikkus'.*

---

**Legalis-RS Arendusmeeskond**
Versioon 0.2.0 | 2024
