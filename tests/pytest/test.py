# -*- coding: utf-8 -*-

from functools import reduce
import random
import sys
import os
import redis
import json
from RLTest import Env
from includes import *
from redis.client import NEVER_DECODE
from RLTest import Defaults

Defaults.decode_responses = True

# ----------------------------------------------------------------------------------------------

# Path to JSON test case files
HERE = os.path.abspath(os.path.dirname(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../.."))
TESTS_ROOT = os.path.abspath(os.path.join(HERE, ".."))

def assertOk(r, x, msg=None):
    r.assertOk(x, message=msg)

def assertExists(r, key, msg=None):
    r.assertTrue(r.exists(key), message=msg)

def assertNotExists(r, key, msg=None):
    r.assertFalse(r.exists(key), message=msg)

def testExample(env):
    """Test that setting the root of a ReJSON key with invalid JSON values fails"""
    r = env

    r.assertEqual(r.execute_command('{{crate_name}}.MUL', '1024'), [1024, 1024])