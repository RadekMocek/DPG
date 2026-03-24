# TUL FM NTI/DPG – Distribuované programování

<!------------------------------------------------------------------------------------------------------>
<!------------------------------------------------------------------------------------------------------>

<details>
<summary><code>./a_restaurace</code></summary>

## Restaurace

```text
Vytvořte aplikaci využívající synchronizační primitiva k vytvoření aplikace,
která bude simulovat provoz v restauraci:

* Máme stůl, kde je 5 hostů.
* Máme 3 chody – polévku, hlavní jídlo a zákusek, ty budou servírovány v tomto pořadí.
* V restauraci je 7 číšníků. Současně obsluhovat ale může maximálně tolik číšníků, kolik je hostů.
* Proces servírování je v následujících krocích:
  * Číšník bere jídlo v kuchyni – prodleva 100 ms
  * Číšník servíruje – prodleva 100 ms
  * Číšník si jde oddechnout – prodleva 500 ms
* Každý host musí dostat postupně vše.
* Host konzumuje každé jídlo 1000 ms.
* Jednoho hosta obsluhuje v dané chvíli jen jeden číšník.
* Aplikace skončí v okamžiku, kdy všichni hosté dostali postupně vše.
* Aplikace bude vypisovat jednotlivé stavy číšníků a hostů:
  > číšník X bere JIDLO
  > číšník X servíruje JIDLO pro hosta Y
  > host Y konzumuje JIDLO
```

[Restaurace source](./a_restaurace/src/main.rs)
</details>

<!------------------------------------------------------------------------------------------------------>
<!------------------------------------------------------------------------------------------------------>

<details>
<summary><code>./b_sorting</code></summary>

## Sortovací algoritmy

```text
Vytvořte aplikaci která bude implementovat vám již námé 2 řadící algoritmy,
QuickSort a MergeSort, s následujícími parametry:

* Vygenerujete si pole či obdobnou kolekci 1000 náhodných čísel v rozmezí 1–10000.
* Vygenerované hodnoty budou jedinečné.
* Implementujte 4 algoritmy:
  * QuickSort v jednom vlákně
  * QuickSort jako multithread
  * MergeSort v jednom vlákně
  * MergeSort jako multithread
* Pomocí každého algoritmu seřaďte výše uvedenou řadu hodnot.
* Pro každý algoritmus zaznamenejte dobu trvání běhu.
* Zkontrolujte seřazené hodnoty a srovnejte časy běhu jednotlivých algoritmů.
```

[Sortovací algoritmy source](./b_sorting/src/)
</details>

<!------------------------------------------------------------------------------------------------------>
<!------------------------------------------------------------------------------------------------------>

<details>
<summary><code>./c_mpi</code></summary>

Projekt pro ověření funkčnosti lokálního MPI.

__Linux:__

```bash
sudo apt install libopenmpi-dev openmpi-bin clang
cargo build
mpirun -np 2 ./target/debug/c_mpi
```

__Windows:__

* Nainstalovat [Microsoft MPI](https://learn.microsoft.com/en-us/message-passing-interface/microsoft-mpi)
  * msmpisetup.exe __a__ msmpisdk.msi
* Nainstalovat [LLVM](https://releases.llvm.org/download.html)
  * Zaškrtnout _Add LLVM to the system PATH_

```powershell
cargo build
mpiexec -np 2 .\target\debug\c_mpi.exe
```

<!------------------------------------------------------------------------------------------------------>
<!------------------------------------------------------------------------------------------------------>

[Source](./c_mpi/src/)
</details>

<details>
<summary><code>./d_sorting_mpi</code></summary>

Porovnání řazení: sekvenčně, vícevláknově, MPI.

[Source](./d_sorting_mpi/src/)
</details>

<!------------------------------------------------------------------------------------------------------>
<!------------------------------------------------------------------------------------------------------>

<details>
<summary><code>./e_matmul</code></summary>

* Vygenerujete si dvě různé celočíselné matice (N x N).
* Implementujte násobení matic: v jednom vlákně, ve více vláknech, v MPI.
* Pro každý algoritmus zaznamenejte dobu trvání běhu.

[Source](./e_matmul/src/)
</details>

## Výsledky

### Merge sort

* Pracuje in-place, ale pro každý merge si alokuje dodatečnou paměť odpovídající počtu právě mergovaných prvků
* _Concurrent_ běží souběžně na 8 vláknech
* _MPI_: 7 Workerů, kterým Master pouze zadá práci a pak zmerguje výsledek

Desktop (Win10 + Xeon E3-1270 v3 + 16 GB):

```text
Generated vector of size 100000000

== Sequential ==
Is sorted: true
~ Elapsed: 26.16s

== Concurrent ==
Is sorted: true
~ Elapsed: 7.15s

== MPI ==
Is sorted: true
~ Elapsed: 8.06s
```

### Násobení matic

* Druhou matici si transponuji, pak zip→map→sum
* _Concurrent_ běží souběžně na 8 vláknech
* _MPI_: 7 Workerů a Master, který tentokrát pracuje také

Desktop (Win10 + Xeon E3-1270 v3 + 16 GB):

```text
mx1 created, size 3000x3000
mx2 created, size 3000x3000

=== MXMUL Sequential ===
~ Elapsed: 11.38s

=== MXMUL Concurrent ===
~ Elapsed: 2.86s

=== MXMUL MPI ===
~ Elapsed: 5.31s

=== Check ===
1 == 2 OK
2 == 3 OK
```
