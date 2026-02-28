# Documentation d'installation et d'utilisation

**Comment ça marche?**

## 1. Environement

Pour que ce projet puisse marché sur votre machine, il est **nécessaire** que vous ayez un système d'exploitqtion **Linux**

## 2. Cloner le repositorie github

Avant, il est **nécessaire** qu'on a une version de `git` installé sur notre machine

```bash
# afficher la version de git sur notre terminal ou CMD
git --version
```

Ensuite on effectue la commande suivante pour cloner le projet

- si on utilise une `ssh`

  ```bash
  git clone git@github.com:ranto-dev/oxpress.git
  ```

- sinon, on peut le cloner directement en utilisant le protocole `https`

  ```bash
  git clone https://github.com/ranto-dev/oxpress.git
  ```

## 3. Utilisation

Dans le répertoire **release**, vous trouverez un fichier executable nommé [**oxpress**](../release/oxpress). Pour la suite, vous pouver suivre les commandes suivante:

```bash
# affichier le guide d'utilisation du cli
./oxpress --help

# pour compresser du fichier texte
./oxpress compress <input.txt> <output.oxp>

# pour restaurer et décompresser un fichier texte depuis un fichier *.oxp
./oxpress decompress <input.oxp> <output.txt>

# pour tester la conformité entre le fichier texte primaire et le fichier apres décompression
sha256sum <primary.txt> <decompressed.txt> # les deux hash doivent etre identique
```

### 4. Demo

#### Compression

<img src="./demo/compress.png" width="100%" />

#### décompression

<img src="./demo/compress.png" width="100%" />

#### Testing and verification

<img src="./demo/verify.png" width="100%" />
