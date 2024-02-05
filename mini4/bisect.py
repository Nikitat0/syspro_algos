import argparse
import os
from subprocess import check_output

def execute(command):
    return os.system(f"{command} >/dev/null 2>&1")

parser = argparse.ArgumentParser()
parser.add_argument('repo_path')
parser.add_argument('first_commit')
parser.add_argument('latest_commit')
parser.add_argument('command', nargs='+')

args = parser.parse_args()

os.chdir(args.repo_path)

git_log_command = f'git log {args.first_commit}..{args.latest_commit} --oneline'
git_log = check_output(git_log_command, shell=True).decode().splitlines()
commits = list(map(lambda s: s.split(maxsplit=1)[0], git_log))
commits.append(args.first_commit)

def is_bad(commit):
    execute(f'git checkout {commit}')
    r = execute(' '.join(args.command)) != 0
    execute('git checkout -')
    return r

l = 0
r = len(commits)
while l + 1 != r:
    pivot = (l + r) // 2
    if is_bad(commits[pivot]):
        l = pivot
    else:
        r = pivot

if is_bad(commits[l]):
    print(f"Commit {commits[l]} is first bad")
else:
    print(f"No bad commits")

execute('git checkout -')
