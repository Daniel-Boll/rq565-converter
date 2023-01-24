## TODO

- [X] Organize the code structure in modules
- [X] Encoder
- [X] Decoder
- [X] Add metadata to .rq565 file
  - [X] Add the width, height to the .rq565 file
  - [X] Add the magic number to the .rq565 file
  - [X] Add a parser for the buffer
    - [X] TryInto or Into
- [X] Add a augment option to decode
- [X] Render the image in a screen

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
