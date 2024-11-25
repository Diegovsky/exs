# Como rodar
Para que não seja necessário que você tenha instalado um ambiente de desenvolvimento rust, incluí binários pré compilados para linux-x86_64 na pasta `bin/`.

Dito isso, aqui estão descrições de como rodá-los:

```
ex1 <arquivo>
binstr [tamanho]
perm [tamanho]
knapsack <arquivo> [first|best] [max_steps]
tsp <arquivo> [first|best] [max_steps]

```

Incluí o arquivo de exemplo do exercício, salvo como `input.txt`.

# Como compilar (opcional)
Você vai precisar dos programas `rustc` e `cargo`. Normalmente podem ser instalados nas distribuições linux pelo pacote `rust`.

Basta executar `cargo build`, e após isso, os programas estarão localizados em `target/debug/`.
