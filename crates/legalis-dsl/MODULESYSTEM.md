# Module System Documentation

The Legalis DSL now supports a comprehensive module system (v0.1.4) for organizing legal documents into modular, reusable structures.

## Features

### 1. Namespace Declarations

Declare a namespace for your module:

```legalis
NAMESPACE tax.income.2024

STATUTE income-tax-rate: "Federal Income Tax Rate" {
    ...
}
```

### 2. Import Statements

#### Simple Import
```legalis
IMPORT "common/definitions.legalis"
IMPORT "tax/credits.legalis" AS credits
```

#### Wildcard Import
Import all public declarations from a module:
```legalis
IMPORT tax.deductions.*
```

#### Selective Import
Import specific items from a module:
```legalis
IMPORT { StandardDeduction, ItemizedDeduction } FROM tax.deductions
IMPORT { EarnedIncomeCred, ChildTaxCredit } FROM tax.credits
```

### 3. Export Declarations

#### Export Single Item
```legalis
EXPORT income-tax-brackets
```

#### Export Multiple Items
```legalis
EXPORT { tax-rate, standard-deduction, filing-status }
```

#### Export All (Wildcard)
```legalis
EXPORT *
```

#### Re-export from Another Module
```legalis
EXPORT { StandardDeduction } FROM tax.deductions
EXPORT * FROM common.definitions
```

### 4. Visibility Modifiers

Control which statutes can be imported by other modules:

#### Public Statute
```legalis
PUBLIC STATUTE federal-tax-rate: "Federal Tax Rate" {
    WHEN income BETWEEN 0 AND 50000
    THEN GRANT "10% tax rate"
}
```

#### Private Statute (default)
```legalis
PRIVATE STATUTE internal-calculation: "Internal Helper" {
    ...
}

// Or simply omit the modifier (defaults to private)
STATUTE helper: "Helper Statute" {
    ...
}
```

## Complete Example

```legalis
// File: tax/income/federal.legalis
NAMESPACE tax.income.federal

// Import dependencies
IMPORT { TaxBracket, FilingStatus } FROM tax.definitions
IMPORT tax.deductions.*

// Export public API
EXPORT { calculate-federal-tax, standard-deduction-2024 }

// Public statute - can be imported by other modules
PUBLIC STATUTE calculate-federal-tax: "Calculate Federal Income Tax" {
    WHEN income > 0 AND filing-status IN ("single", "married")
    THEN GRANT "tax calculation"
}

// Public statute
PUBLIC STATUTE standard-deduction-2024: "Standard Deduction for 2024" {
    DEFAULT amount 14600
    WHEN filing-status = "single"
    THEN GRANT "standard deduction"
}

// Private statute - internal use only
PRIVATE STATUTE internal-tax-helper: "Internal Tax Helper" {
    WHEN income > 100000
    THEN GRANT "additional calculation"
}
```

```legalis
// File: tax/credits.legalis
NAMESPACE tax.credits

// Re-export common definitions
EXPORT { TaxCredit, RefundableCredit } FROM tax.definitions

PUBLIC STATUTE earned-income-credit: "Earned Income Tax Credit" {
    WHEN income BETWEEN 0 AND 59187 AND has children
    THEN GRANT "EITC eligibility"
}

PUBLIC STATUTE child-tax-credit: "Child Tax Credit" {
    WHEN has qualifying-child AND income < 200000
    THEN GRANT "child tax credit"
}
```

```legalis
// File: main.legalis
NAMESPACE tax.main

// Import from federal module
IMPORT { calculate-federal-tax, standard-deduction-2024 } FROM tax.income.federal

// Import all credits
IMPORT tax.credits.*

// Use imported statutes...
```

## Implementation Details

### AST Structures

- **NamespaceNode**: Represents `NAMESPACE path` declarations
- **ImportNode**: Enhanced with `ImportKind` enum (Simple, Wildcard, Selective)
- **ExportNode**: Represents `EXPORT` declarations with optional re-export
- **Visibility**: Enum for Public/Private modifiers on statutes

### Parser Methods

- `parse_namespace()`: Parses namespace declarations
- `parse_import()`: Enhanced to handle all import styles
- `parse_export()`: Parses export declarations
- `parse_statute_node()`: Enhanced to parse visibility modifiers

## Benefits

1. **Organization**: Group related legal rules into modules
2. **Reusability**: Import common definitions across multiple documents
3. **Encapsulation**: Control API surface with visibility modifiers
4. **Maintainability**: Update shared rules in one place
5. **Namespace Management**: Avoid naming conflicts with hierarchical namespaces

## Future Enhancements

- Module path resolution and loading
- Circular dependency detection
- Module versioning and compatibility
- IDE support for auto-import and completion
