from utils import *

triplet_regex = re.compile(r'(.)\1\1')
Entry = namedtuple('Entry', ['value', 'quintuplet', 'index_added'])

def part1(salt):
    return search_for_hashes(all_hashes(salt))

def part2(salt):
    return search_for_hashes(all_stretched_hashes(salt))

def search_for_hashes(hashlist):
    # Keep a list of patterns we are searching for, and when these
    # patterns were added
    searches = []
    keys = []

    for (index, h) in enumerate(hashlist):
        searches = [s for s in searches if is_search_valid(s, index)]
        new_searches = list(searches)

        for entry in searches:
            if re.search(entry.quintuplet, h):
                keys.append(entry)
                new_searches.remove(entry)

        searches = new_searches

        found_triplet = triplet_regex.search(h)
        if found_triplet:
            char = found_triplet.group(1)
            new_entry = Entry(h, char * 5, index)
            searches.append(new_entry)

        if len(keys) >= 64:
            return keys[63].index_added

def is_search_valid(entry, current_index):
    return current_index <= entry.index_added + 1001

def all_stretched_hashes(salt):
    # Apply another layer over the all_hashes generator, which takes each hash
    # and 'stretches' it by re-hashing it 2016 times

    def get_stretched_hash(h):
        current_hash = h
        for i in range(2016):
            current_hash = hash_utf(current_hash)
        return current_hash

    return map(get_stretched_hash, all_hashes(salt))

def all_hashes(salt):
    natural_numbers_as_strings = map(str, count())

    salt_with_natural_numbers = zip_with_constant(salt, natural_numbers_as_strings)

    # An infinite generator of all strings of the form salt0, salt1, etc.
    input_strings = map(concat, salt_with_natural_numbers)

    # An infinite generator of all hashed inputs
    all_hashed_strings = map(hash_utf, input_strings)

    return all_hashed_strings
