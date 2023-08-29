## Milestone 4: Factoring Numbers

The goals of this milestone are to write two functions that return a number's prime factors, using a "slow" and a fast (using a sieve, see previous milestone) method.

### Usage

Example:

```shell
❯ cargo run
   ...

Num: 312680865509917
find_factors: 40.647558ms seconds
7791799 40129483
Product: 312680865509917

find_factors_sieve: 17.629231ms seconds
7791799 40129483
Product: 312680865509917

Num: 1819448968910731
find_factors: 210.341561ms seconds
40129483 45339457
Product: 1819448968910731

find_factors_sieve: 40.672328ms seconds
40129483 45339457
Product: 1819448968910731

Num: 6795742697625173
find_factors: 67.276862ms seconds
6880691 987654103
Product: 6795742697625173

find_factors_sieve: 75.291607ms seconds
6880691 987654103
Product: 6795742697625173

Num: 64374108854777
find_factors: 45.454665ms seconds
64374108854777
Product: 64374108854777

find_factors_sieve: 8.372458ms seconds
64374108854777
Product: 64374108854777

Num: 0
❯
```
