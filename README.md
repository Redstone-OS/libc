# LibC - Redstone OS

C Library para Redstone OS. Fornece wrappers para syscalls do kernel.

## Funcionalidades

- ✅ Syscall `write` - Escrever em stdout/stderr
- ✅ Syscall `exit` - Terminar processo
- ⏳ Syscall `read` - Ler de stdin (TODO)
- ⏳ Syscall `open` - Abrir arquivo (TODO)

## Uso

```rust
use libc::{print, println, exit};

fn main() {
    println("Hello from Redstone OS!");
    exit(0);
}
```

## Syscalls Implementadas

### write(fd, buf)
Escreve dados em um file descriptor.

**Argumentos:**
- `fd`: File descriptor (1 = stdout, 2 = stderr)
- `buf`: Buffer com dados

**Retorno:** Número de bytes escritos

### exit(code)
Termina o processo.

**Argumentos:**
- `code`: Código de saída (0 = sucesso)

## Compilação

```bash
cargo build --release --target x86_64-unknown-none
```

## TODO

- [ ] Implementar mais syscalls (read, open, close)
- [ ] Adicionar suporte a formatação (format!)
- [ ] Implementar errno
- [ ] Adicionar testes
