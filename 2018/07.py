from utils import *

def find_parent(relations, node):
    for parent, children in relations.items():
        if node in children:
            return parent

    # Node is the root (no parent found)
    return None

def find_root(relations):
    all_nodes = set(relations.keys()) | set(flatten(relations.values()))
    roots = [node for node in all_nodes if find_parent(relations, node) is None]

    return roots

def part1(relations):
    relations = relations.copy()

    order = []

    while True:
        # Ties are broken alphabetically (lexicographically minimal)
        next_job = min(find_root(relations))
        order.append(next_job)
        relations.pop(next_job)

        if len(relations) == 0:
            break

    return "".join(order)


def part2(relations):
    relations = relations.copy()

    number_of_workers = 5
    base_time_per_job = 60

    order = []
    time = 0

    # Stores { worker_number: (task, finish_time) }
    workers = {}
    for i in range(number_of_workers):
        workers[i] = None

    def job_is_in_progress(job):
        info_of_jobs_in_progress = [job_info for job_info in workers.values() if job_info is not None]
        return job in (j for j, finish_time in info_of_jobs_in_progress)

    def time_to_finish(job):
        return base_time_per_job + (ord(job) - ord('A'))

    def try_assign_job(worker):
        available_jobs = [job for job in find_root(relations) if not job_is_in_progress(job)]

        if not available_jobs:
            return None

        next_job = min(available_jobs)
        workers[worker] = (next_job, time + time_to_finish(next_job))
        return next_job

    while True:
        jobs_finished_this_step = []

        # First, try to assign jobs.
        # Looks for available jobs that aren't in progress (if there are any)
        for worker, worker_job in workers.items():
            if worker_job is None:
                new_job = try_assign_job(worker)

        # After jobs have been assigned, check if any have been finished this tick.
        # We can't do this at the same time as the above otherwise workers may
        # pick up finished jobs that haven't been expired by the code below.
        for worker, worker_job in workers.items():
            if worker_job is not None:
                job, finish_time = worker_job
                if finish_time == time:
                    jobs_finished_this_step.append((worker, job))

        # Expire any jobs that finished this tick
        for worker, job in jobs_finished_this_step:
            relations.pop(job)
            workers[worker] = None

        if len(relations) == 0:
            # Account for the single extra tick when neither worker is doing anything
            return time + 1

        time += 1

def transform_input(i):
    lines = i.splitlines()

    def extract_line(line):
        groups = re.search('Step ([A-Z]) must be finished before step ([A-Z]) can begin.', line).groups()
        return (groups[0], groups[1])

    lines = [extract_line(line) for line in lines]

    relations = defaultdict(set)
    for a, b in lines:
        relations[a].add(b)

    all_nodes = set(relations.keys()) | set(flatten(relations.values()))
    for node in all_nodes:
        if node not in relations:
            relations[node] = set()

    return relations
