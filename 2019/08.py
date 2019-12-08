from utils import *

image_width = 25
image_height = 6
pixels_per_image = image_height * image_width

def part1(layers):
    layer_with_least_zeros, _ = argmin({ layer: layer.count('0') for layer in layers })
    return layer_with_least_zeros.count('1') * layer_with_least_zeros.count('2')

def part2(layers):
    # Grab pixels from every layer for each pixel
    pixels = ["".join(layer[pixel_index] for layer in layers)
              for pixel_index in range(pixels_per_image)]

    # Grab the most significant pixel from each set of pixels for layers
    pixels = [next(filter(lambda x: x != '2', layer)) for layer in pixels]

    # Divide the pixels into lines
    pixel_lines = list(chunk_list(pixels, image_width))

    # Format for the terminal
    result = "\n" + "\n".join("".join(line).replace('0', ' ').replace('1', '*') for line in pixel_lines)
    return result

def transform_input(i):
    return list(chunk_list(i, pixels_per_image))
