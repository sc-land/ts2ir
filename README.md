# TS2IR - TypeScript to IR Converter

Um parser simples de TypeScript para Rust que converte código TypeScript em uma representação intermediária (IR).

## Funcionalidade

Este projeto implementa um parser básico que pode processar classes TypeScript simples, como:

```typescript
class Cat {
    energy: number;
    breath: number;
}
```

## Como usar

### Compilar o projeto

```bash
cargo build
```

### Executar o parser

```bash
./target/debug/ts2ir <arquivo.ts>
```

Exemplo:
```bash
./target/debug/ts2ir example.ts
```

### Executar testes

```bash
cargo test
```

## Estrutura do projeto

- `src/main.rs` - Aplicação principal com o parser TypeScript
- `src/lib.rs` - Biblioteca base
- `example.ts` - Arquivo de exemplo para teste
- `Cargo.toml` - Configuração do projeto Rust

## Saída

O parser produz duas representações:

1. **Estrutura Rust**: Uma representação interna usando structs Rust
2. **JSON IR**: Uma representação JSON da classe parseada

Exemplo de saída:
```rust
ClassDefinition {
    name: "Cat",
    properties: [
        Property {
            name: "energy",
            type_name: "number",
        },
        Property {
            name: "breath",
            type_name: "number",
        },
    ],
}
```

```json
{
  "type": "class",
  "name": "Cat",
  "properties": [
    {
      "name": "energy",
      "type": "number"
    },
    {
      "name": "breath",
      "type": "number"
    }
  ]
}
```

## Dependências

- `serde` - Para serialização JSON
- `serde_json` - Para manipulação JSON
- `ts` - Módulo local para parsing TypeScript
- `ir` - Módulo local para representação intermediária

## Limitações atuais

Este é um parser básico que suporta apenas:
- Classes simples
- Propriedades com tipos primitivos
- Sintaxe TypeScript básica

Funcionalidades mais avançadas podem ser adicionadas conforme necessário.
