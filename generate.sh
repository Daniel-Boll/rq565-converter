#!/bin/bash

# Create an array of all combinations of mask
mask=(088 808 880 565)

# Loop through the array
for i in "${mask[@]}"; do
  # Encode the image with the mask
  cargo run -- encode -i assets/three_colors.png -f "$i" -o "output_$i.rq565"

  # Decode the image with the mask
  cargo run -- decode -i "output_$i.rq565" -o "output_$i.png"
done

feh output_*.png
