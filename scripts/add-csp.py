#!/usr/bin/env python3
"""Roda DEPOIS do `trunk build --release`.

O Trunk coloca um <script> inline no dist/index.html para iniciar o WebAssembly, e o hash
dele muda a cada build. Este script calcula o hash (sha256) de todo script inline e grava
uma Content-Security-Policy no <head>, sem precisar de 'unsafe-inline' em script-src.

Uso: python3 scripts/add-csp.py [dist/index.html]
"""
import base64
import hashlib
import re
import sys
from pathlib import Path

path = Path(sys.argv[1] if len(sys.argv) > 1 else "dist/index.html")
html = path.read_text(encoding="utf-8")

# <script ...>conteúdo</script> sem atributo src = script inline
hashes = []
for m in re.finditer(r"<script(?P<attrs>[^>]*)>(?P<body>.*?)</script>", html, re.S | re.I):
    if re.search(r"\bsrc\s*=", m.group("attrs"), re.I) or not m.group("body").strip():
        continue
    digest = hashlib.sha256(m.group("body").encode("utf-8")).digest()
    hashes.append("'sha256-" + base64.b64encode(digest).decode() + "'")

if not hashes:
    sys.exit("Nenhum script inline encontrado: o formato do Trunk mudou? Revise antes de publicar.")

script_src = " ".join(["'self'", "'wasm-unsafe-eval'", *hashes])
csp = "; ".join([
    "default-src 'self'",
    f"script-src {script_src}",
    # 'unsafe-inline' só em estilo: a página usa style="--i: N" nos elementos
    "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
    "font-src 'self' https://fonts.gstatic.com",
    "img-src 'self' data:",
    "connect-src 'self'",
    "frame-src https://open.spotify.com",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'none'",
])

tag = f'<meta http-equiv="Content-Security-Policy" content="{csp}" />'
if "Content-Security-Policy" in html:
    sys.exit("O index.html já tem uma CSP; nada a fazer.")

# A CSP precisa vir antes de qualquer script/estilo, então entra logo depois de <head>.
new_html, n = re.subn(r"(<head[^>]*>)", lambda m: m.group(1) + "\n    " + tag, html, count=1, flags=re.I)
if n != 1:
    sys.exit("Não achei <head> no index.html.")

path.write_text(new_html, encoding="utf-8")
print(f"CSP gravada em {path} com {len(hashes)} hash(es) de script inline.")
