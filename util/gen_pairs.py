import itertools

def generate_vectors(filename):
    range_numbers = range(-9, 10)

    vectors = itertools.product(range_numbers, repeat=4)

    with open(filename, 'w') as file:
        for vector in vectors:
            file.write(f"{list(vector)},\n")

output_file = "vectors.txt"

generate_vectors(output_file)

print(f"Vectors written to {output_file}")