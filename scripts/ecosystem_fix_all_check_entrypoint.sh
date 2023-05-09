#!/bin/bash
# Wrapper for ecosystem_fix_all_check.py

if [ ! -d ".venv/bin" ]; then
  python -m venv .venv
  .venv/bin/pip install tqdm
fi

.venv/bin/python ecosystem_fix_all_check.py
