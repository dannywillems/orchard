window.BENCHMARK_DATA = {
  "lastUpdate": 1779893487921,
  "repoUrl": "https://github.com/dannywillems/orchard",
  "entries": {
    "Orchard Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "ewillbefull@gmail.com",
            "name": "Sean Bowe",
            "username": "ebfull"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c6c93d5e43959d143906815e4107f9a4cd3d5850",
          "message": "Merge pull request #496 from valargroup/valar/drop-legacy-fixed-point-impls\n\nrefactor: collapse OrchardFixedBases and drop dead FixedPoint impls",
          "timestamp": "2026-04-27T12:41:58-06:00",
          "tree_id": "448ac9686f535bdbfabcb5865e32881d64e60d3c",
          "url": "https://github.com/dannywillems/orchard/commit/c6c93d5e43959d143906815e4107f9a4cd3d5850"
        },
        "date": 1779893486474,
        "tool": "cargo",
        "benches": [
          {
            "name": "proving/bundle/1",
            "value": 2588633146,
            "range": "± 200280731",
            "unit": "ns/iter"
          },
          {
            "name": "proving/bundle/2",
            "value": 2590483581,
            "range": "± 5693460",
            "unit": "ns/iter"
          },
          {
            "name": "proving/bundle/3",
            "value": 3708006599,
            "range": "± 4810922",
            "unit": "ns/iter"
          },
          {
            "name": "proving/bundle/4",
            "value": 4821395541,
            "range": "± 17580323",
            "unit": "ns/iter"
          },
          {
            "name": "verifying/bundle/1",
            "value": 20462195,
            "range": "± 582185",
            "unit": "ns/iter"
          },
          {
            "name": "verifying/bundle/2",
            "value": 20679159,
            "range": "± 176993",
            "unit": "ns/iter"
          },
          {
            "name": "verifying/bundle/3",
            "value": 23780819,
            "range": "± 198212",
            "unit": "ns/iter"
          },
          {
            "name": "verifying/bundle/4",
            "value": 26936709,
            "range": "± 199067",
            "unit": "ns/iter"
          },
          {
            "name": "note-decryption/valid",
            "value": 1480022,
            "range": "± 9251",
            "unit": "ns/iter"
          },
          {
            "name": "note-decryption/invalid",
            "value": 124253,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "note-decryption/compact-valid",
            "value": 1475459,
            "range": "± 4898",
            "unit": "ns/iter"
          },
          {
            "name": "compact-note-decryption/invalid",
            "value": 1312194157,
            "range": "± 1705178",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/valid/10",
            "value": 15607976,
            "range": "± 22702",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/invalid/10",
            "value": 2105895,
            "range": "± 3588",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-valid/10",
            "value": 15565718,
            "range": "± 136985",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-invalid/10",
            "value": 2071055,
            "range": "± 4639",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/valid/50",
            "value": 77937211,
            "range": "± 83542",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/invalid/50",
            "value": 10474741,
            "range": "± 18973",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-valid/50",
            "value": 77755946,
            "range": "± 138788",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-invalid/50",
            "value": 10298584,
            "range": "± 73727",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/valid/100",
            "value": 155875737,
            "range": "± 205922",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/invalid/100",
            "value": 20942795,
            "range": "± 37321",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-valid/100",
            "value": 155556590,
            "range": "± 249288",
            "unit": "ns/iter"
          },
          {
            "name": "batch-note-decryption/compact-invalid/100",
            "value": 20597525,
            "range": "± 35057",
            "unit": "ns/iter"
          },
          {
            "name": "derive_fvk",
            "value": 453741,
            "range": "± 2368",
            "unit": "ns/iter"
          },
          {
            "name": "default_address",
            "value": 489192,
            "range": "± 815",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}