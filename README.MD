# TUL FM NTI/DPG – Distribuované programování

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
