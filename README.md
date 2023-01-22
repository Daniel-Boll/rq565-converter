## TODO

- [X] Organize the code structure in modules
- [ ] Encoder
- [ ] Decoder

# Objetivo

Implementar o método de quantização de imagens coloridas para MxNxO bits.
  1) O sistema deve abrir uma imagem qualquer (24 bits), quantiza-la para MxNxO bits (M+N+O = 16) 
  e salvá-la em disco, usando um formato proprietário (crie o seu próprio formato).

  2) O sistema deve abrir a imagem salva em formato proprietário (o seu formato próprio) e 
  salvá-la em disco em um formato popular de livre escolha (sem compressão: BMP, PNG ou outro).
---

# Especificação

image.rq565 - formato de imagem próprio "RustQuant565 Image Format"
  M - número de bits para a cor vermelha (5)
  N - número de bits para a cor verde (6)
  O - número de bits para a cor azul (5)
