#!/usr/bin/python3
"""Offline GitHub CLI double. Every side effect is a JSON file in the fixture directory."""
import json
import os
from pathlib import Path
import sys
import time

state = Path(os.environ['REQVIRE_GH_TEST_STATE'])
config = json.loads((state / 'scenario.json').read_text())
args = sys.argv[1:]
body = sys.stdin.read()
with (state / 'calls.jsonl').open('a') as log:
    log.write(json.dumps({'args': args, 'body': body, 'prompt_disabled': os.environ.get('GH_PROMPT_DISABLED')}) + '\n')
operation = ' '.join(args[:2]) if args[0] in ('auth', 'repo') else ' '.join(args[:2]) if args[0] == 'pr' else args[0]
if config.get('fail') == operation:
    print('HTTP 403 denied (secret-token-must-not-escape)', file=sys.stderr)
    sys.exit(1)
if config.get('timeout') == operation:
    time.sleep(30)
if config.get('malformed') == operation:
    print('invalid structured data')
    sys.exit(0)
def option(name):
    return args[args.index(name) + 1]
if args == ['--version']:
    print('gh version 2.80.0 (fixture)')
elif operation == 'auth status':
    assert '--active' in args and '--hostname' in args
elif operation == 'repo view':
    print(json.dumps({'nameWithOwner': config.get('repository', 'test/project'), 'viewerPermission': config.get('permission', 'READ')}))
elif args[0] == 'api':
    print(json.dumps({'ahead_by': config.get('ahead_by', 1)}))
elif operation == 'pr list':
    path = state / 'prs.json'
    print(path.read_text() if path.exists() else '[]')
elif operation == 'pr create':
    pr = {'number': 7, 'url': 'https://github.com/test/project/pull/7', 'headRefName': option('--head'),
          'baseRefName': option('--base'), 'isDraft': '--draft' in args, 'isCrossRepository': False}
    (state / 'prs.json').write_text(json.dumps([pr]))
    if config.get('lost_create'):
        print('connection reset after creation', file=sys.stderr)
        sys.exit(1)
    print(pr['url'])
elif operation == 'pr view':
    number = int(args[2])
    print(json.dumps({'number': number, 'url': f'https://github.com/test/project/pull/{number}'}))
elif operation == 'pr comment':
    with (state / 'comments.jsonl').open('a') as comments:
        comments.write(json.dumps({'number': int(args[2]), 'body': body}) + '\n')
    if config.get('lost_comment'):
        print('connection reset after posting', file=sys.stderr)
        sys.exit(1)
    print(f'https://github.com/test/project/pull/{args[2]}#issuecomment-123')
else:
    raise AssertionError(f'Unexpected gh command: {args}')
