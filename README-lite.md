# gcfeeder-lite

🇬🇧 [English](#english) · 🇫🇷 [Français](#français)

---

<a name="english"></a>
# 🇬🇧 English

A fork of [Sirius902/gcfeeder](https://github.com/Sirius902/gcfeeder) that adds a **headless build** using almost 0% CPU, designed for low-end processors.

> Original project by Sirius902. This fork adds `gcfeeder-lite` and optimizations in `gcfeeder-core`. Changes written with the help of Claude Sonnet 5.5 (Anthropic).

## Why

The GUI version repaints its window continuously and wakes about ten threads 125 times per second. On an i5-6600 this used 25% of the CPU (one core out of four), even when the controller was idle.

## What changed

- **`gcfeeder-lite`**: an executable with no window and no GPU usage that reads the same `gcfeeder.toml`.
- **`gcfeeder-core`** (also benefits the GUI version):
  - rayon thread pools removed; USB read then write run sequentially;
  - rumble is sent to the adapter only when it changes;
  - no ViGEm update when the controller state is unchanged;
  - retries back off (8 ms up to 1 s) when the adapter or ViGEm is missing.
- **GUI**: repaints at about 15 frames per second instead of a tight loop, and not at all while hidden in the tray.

The lite build has no interface, no graphical calibration and no UDP server for gcviewer.

## Requirements

- Windows 10 or 11.
- GameCube adapter (WUP-028) with the **WinUSB** driver installed through [Zadig](https://zadig.akeo.ie) (guide: [Dolphin](https://dolphin-emu.org/docs/guides/how-use-official-gc-controller-adapter-wii-u)).
- [ViGEmBus](https://github.com/ViGEm/ViGEmBus/releases).

## Usage

1. Put `gcfeeder-lite.exe` and `gcfeeder.toml` in the same folder. The `.toml` is optional: without it, defaults are used (Xbox 360, `stick_click`, rumble on).
2. Run in a terminal, choosing the adapter port(s) your controller is plugged into:

```
gcfeeder-lite.exe --ports 1
gcfeeder-lite.exe --ports 3
gcfeeder-lite.exe --ports 1,2
```

| Option | Effect |
| --- | --- |
| `--ports <list>` | ports to feed, from 1 to 4, e.g. `1`, `3` or `1,2` (default: 1,2,3,4). Feeding only the ports you use is the most economical. |
| `--config <path>` | use another `gcfeeder.toml` |
| `--verbose` | also write info messages to the log |
| `-h`, `--help` | show help |

Warnings and errors are written to `gcfeeder-lite.log`, next to the executable.

When built with `--features background` there is no window at all. To stop it:

```
taskkill /im gcfeeder-lite.exe /f
```

## Calibrating or changing settings

The lite build has no interface. To change the configuration:

1. Stop the lite build.
2. Run the regular `gcfeeder.exe` from the same folder, calibrate and change settings, then **save**.
3. Fully close gcfeeder (File > Exit): only one program can use the adapter at a time.
4. Start the lite build again. It only reads the file at startup.

The lite build uses the profile selected for each port.

## Start with Windows

1. Press Win+R, type `shell:startup` and press Enter.
2. In the folder that opens, right-click > New > Shortcut, and browse to `gcfeeder-lite.exe`.
3. Right-click the shortcut > Properties, and append the ports you want to the **Target** field, **after** the closing quote of the path, with a space. Use any port from 1 to 4, or several:

```
"C:\path\to\gcfeeder-lite.exe" --ports 1
"C:\path\to\gcfeeder-lite.exe" --ports 4
"C:\path\to\gcfeeder-lite.exe" --ports 1,2
```

4. Set **Start in** to the folder containing `gcfeeder.toml` if it is not already.

## Building

1. Install [Rust](https://rustup.rs) and the Visual Studio Build Tools ("Desktop development with C++" workload).
2. In the repository folder:

```
cargo build --release -p gcfeeder-lite --features background
```

3. The executable is at `target\release\gcfeeder-lite.exe`.

Without `--features background`, a console window stays open.

## Troubleshooting

- **The controller does not respond**: make sure no other `gcfeeder.exe` or Dolphin passthrough is using the adapter, check that `--ports` matches the port your controller is plugged into, then read `gcfeeder-lite.log`.
- **"Failed to connect to adapter"**: the WinUSB driver is missing or the adapter is unplugged.
- **"Failed to connect to bridge"**: ViGEmBus is not installed.
- **Antivirus flags the executable**: it is not code-signed. Build it yourself from source if in doubt.

## License and credits

The original repository declares no license. This fork does not add one and remains subject to the rights of the original author, **Sirius902**.

---

<a name="français"></a>
# 🇫🇷 Français

Fork de [Sirius902/gcfeeder](https://github.com/Sirius902/gcfeeder) qui ajoute une **version sans interface** utilisant presque 0 % de CPU, pensée pour les petits processeurs.

> Projet d'origine : Sirius902. Ce fork ajoute `gcfeeder-lite` et des optimisations dans `gcfeeder-core`. Modifications écrites avec l'aide de Claude Sonnet 5.5 (Anthropic).

## Pourquoi

La version GUI redessine sa fenêtre en continu et réveille une dizaine de threads 125 fois par seconde. Sur un i5-6600, cela représentait 25 % du CPU (un cœur sur quatre), même sans toucher à la manette.

## Ce qui change

- **`gcfeeder-lite`** : exécutable sans fenêtre ni GPU, qui lit le même `gcfeeder.toml`.
- **`gcfeeder-core`** (profite aussi à la version GUI) :
  - plus de pools rayon ; lecture puis écriture USB en séquentiel ;
  - vibration envoyée à l'adaptateur seulement quand elle change ;
  - pas de mise à jour ViGEm si l'état de la manette est identique ;
  - réessais espacés (de 8 ms à 1 s) quand l'adaptateur ou ViGEm est absent.
- **GUI** : redessin à environ 15 images par seconde au lieu d'une boucle serrée, et rien quand la fenêtre est masquée dans la barre des tâches.

Le lite n'a ni interface, ni calibration graphique, ni serveur UDP pour gcviewer.

## Prérequis

- Windows 10 ou 11.
- Adaptateur GameCube (WUP-028) avec le pilote **WinUSB** installé via [Zadig](https://zadig.akeo.ie) (guide : [Dolphin](https://dolphin-emu.org/docs/guides/how-use-official-gc-controller-adapter-wii-u)).
- [ViGEmBus](https://github.com/ViGEm/ViGEmBus/releases).

## Utilisation

1. Place `gcfeeder-lite.exe` et `gcfeeder.toml` dans le même dossier. Le `.toml` est facultatif : sans lui, les réglages par défaut sont utilisés (Xbox 360, `stick_click`, vibration activée).
2. Lance dans un terminal, en choisissant le ou les ports de l'adaptateur où ta manette est branchée :

```
gcfeeder-lite.exe --ports 1
gcfeeder-lite.exe --ports 3
gcfeeder-lite.exe --ports 1,2
```

| Option | Effet |
| --- | --- |
| `--ports <liste>` | ports à alimenter, de 1 à 4, par exemple `1`, `3` ou `1,2` (défaut : 1,2,3,4). N'alimenter que les ports utilisés est le plus économe. |
| `--config <chemin>` | utilise un autre `gcfeeder.toml` |
| `--verbose` | écrit aussi les messages d'information dans le log |
| `-h`, `--help` | affiche l'aide |

Les avertissements et erreurs sont écrits dans `gcfeeder-lite.log`, à côté de l'exécutable.

Avec la compilation `--features background`, il n'y a aucune fenêtre. Pour l'arrêter :

```
taskkill /im gcfeeder-lite.exe /f
```

## Calibrer ou changer les réglages

Le lite n'a pas d'interface. Pour modifier la configuration :

1. Arrête le lite.
2. Lance le `gcfeeder.exe` normal depuis le même dossier, calibre et change les réglages, puis **enregistre**.
3. Ferme complètement gcfeeder (File > Exit) : un seul programme peut utiliser l'adaptateur à la fois.
4. Relance le lite. Il ne lit le fichier qu'au démarrage.

Le lite utilise le profil sélectionné pour chaque port.

## Démarrer avec Windows

1. Appuie sur Win+R, tape `shell:startup` puis Entrée.
2. Dans le dossier qui s'ouvre, clic droit > Nouveau > Raccourci, et choisis `gcfeeder-lite.exe`.
3. Clic droit sur le raccourci > Propriétés, et ajoute les ports voulus dans le champ **Cible**, **après** le guillemet fermant du chemin, avec un espace. Tu peux mettre n'importe quel port de 1 à 4, ou plusieurs :

```
"C:\chemin\vers\gcfeeder-lite.exe" --ports 1
"C:\chemin\vers\gcfeeder-lite.exe" --ports 4
"C:\chemin\vers\gcfeeder-lite.exe" --ports 1,2
```

4. Mets le champ **Démarrer dans** sur le dossier contenant `gcfeeder.toml`, s'il n'y est pas déjà.

## Compiler

1. Installe [Rust](https://rustup.rs) et les Build Tools Visual Studio (charge « Développement Desktop en C++ »).
2. Dans le dossier du dépôt :

```
cargo build --release -p gcfeeder-lite --features background
```

3. L'exécutable est dans `target\release\gcfeeder-lite.exe`.

Sans `--features background`, une fenêtre console reste ouverte.

## Dépannage

- **La manette ne répond pas** : vérifie qu'aucun autre `gcfeeder.exe` ni passthrough Dolphin n'utilise l'adaptateur, que `--ports` correspond au port où ta manette est branchée, puis lis `gcfeeder-lite.log`.
- **« Failed to connect to adapter »** : pilote WinUSB manquant ou adaptateur débranché.
- **« Failed to connect to bridge »** : ViGEmBus n'est pas installé.
- **L'antivirus signale l'exécutable** : il n'est pas signé numériquement. Compile-le toi-même depuis les sources si tu as un doute.

## Licence et crédits

Le dépôt d'origine ne déclare aucune licence. Ce fork n'en ajoute pas non plus et reste soumis aux droits de l'auteur d'origine, **Sirius902**.
