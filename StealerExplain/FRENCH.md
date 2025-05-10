# 🇫🇷 Version française

L'objectif de ce document sur ce qu'est un "**stealer**" est de vous faire comprendre à quel point cela peut être **dangereux** et qu'ils peuvent se trouver **partout**.

---

En bref, un "**stealer**" est un programme malveillant qui a pour but de **subtiliser des informations sensibles** à notre égard. Les attaquants peuvent vouloir vos informations pour plusieurs raisons :
 - **Un accès à vos comptes de messagerie** (pour les revendre ou répéter l'attaque en se faisant passer pour vous en utilisant votre email)
 - **Un accès administrateur à votre compte d'entreprise** (pour avoir accès à des dossiers/fichiers sensibles qui peuvent ensuite être utilisés pour du chantage ou simplement revendus)
 - **Des informations sur vos données bancaires** (pour ensuite faire du "phishing" et essayer de vous voler de l'argent)

Dans d'autres cas, cela peut être directement une **demande de rançon**, sinon ils exposeront vos données à, par exemple, tous vos contacts ou sur des sites spécialisés dans le **"doxing"**.

> [!CAUTION]  
> **Ne payez jamais la rançon** (/ ne vous soumettez jamais à ses demandes).  
> La seule chose que l'attaquant fera après avoir reçu le paiement, c'est de **faire fuiter vos données** et de disparaître.  
> Voici des liens utiles pour limiter les dégâts:  
> - [**WebCleaner**](https://www.webcleaner.fr/)
> - [**Incogni**](https://nordvpn.com/fr/incogni/)
> 
> Les plateformes les plus susceptibles d'héberger votre "dox":
> - [**Doxbin**](https://www.doxbin.com/)
> - [**Zerobin**](https://zerobin.net/)
> - [**Pastebin**](https://pastebin.com/)

---

Maintenant, nous allons voir **tout ce qu'un stealer fait**, **où il va chercher les informations** et **comment le hacker les utilise**.

Il faut savoir que les **"stealers" ne sont pas tous les mêmes**, tout cela va dépendre de plusieurs facteurs :

- **Est-ce que l'attaquant l'a codé lui-même** ou **l'a juste acheté**, voire **pris le premier stealer open source disponible sur GitHub** ?
- **Est-ce que l'attaquant a déjà vu** (via une photo ou une vidéo publiée sur un réseau social, par exemple) **votre bureau**, les différents **fichiers/dossiers qu'il contenait**, et **les navigateurs web que vous utilisez** ?
- **Votre système d'exploitation** (*Windows, Linux, macOS*)

Tout ça a son importance car, prenez :

- **Est-ce que l'attaquant l'a codé lui-même** ou **pris un stealer déjà existant** ?

Si l'attaquant l'a codé lui-même, et qu'en plus de ça, il a déjà vu :
- **Votre bureau**
- **Vos navigateurs web**
- **Vos applications**, etc.

Et qu'en plus de ça, il **connaît votre système d'exploitation**, il peut très bien développer **un stealer compatible avec celui-ci**, et **chercher au bon endroit** en optimisant son code pour récupérer **vos logins** et **cookies de navigateur**.

## Comment ça marche ?

Nous allons partir du principe que l'attaquant a utilisé un **stealer open source**.  
Maintenant qu'il a réussi à vous faire télécharger et exécuter son stealer, **quelles sont les données qu'il a en sa possession** ?

Voici une liste des **informations qu'il risque de détenir** :

- **Les informations sur votre système** (MAC, HWID, IP, KEY PRODUCT, etc.)
- **Vos logins** (web)
- **Votre historique** (web)
- **Vos cookies** (web)
- **Vos autofills** (web)
- **Vos bookmarks** (web)
- **Des fichiers spécifiques** (`password.txt`, `mdp.txt`, etc.)
- **Vos wallets**
- **Votre session Discord**
- etc.

> [!NOTE]  
> Vous pouvez vous rendre sur ces deux GitHub pour avoir une vision  
> plus étendue de ce qu'on peut obtenir avec un stealer open source :
> - [CStealer](https://github.com/PIKA-X-777/CStealer)
> - [Doenerium](https://github.com/doenerium6969/doenerium-fixed)
>
> Et ici pour comprendre **où il les récupère** :
> - [CStealer Path](https://github.com/PIKA-X-777/CStealer/blob/main/creal.py#L931)
> - [Doenerium Path](https://github.com/doenerium6969/doenerium-fixed/blob/main/stub/stub.js#L272)

> [!TIP]  
> Pour comprendre **comment un stealer est conçu**,  
> consultez le GitHub de [Lawxsz](https://github.com/Lawxsz/make-u-own-stealer/)  
> (très utile si vous voulez en apprendre plus sur leur fonctionnement).

Pour ceux qui ont du mal avec Python, voici **une explication simplifiée** du fonctionnement.

Les stealers ne font que :

- **Tuer les tâches gênantes** (navigateurs actifs qui bloquent l'accès aux données)
- **Chercher les fichiers ciblés**
- **Décrypter** les données (si nécessaire)
- **Les envoyer** à l'attaquant (webhook, FTP, etc.)

### Pour "tuer les tâches gênantes" :
Le programme **ferme les navigateurs** (Chrome, Edge, Firefox, etc.)  
afin de pouvoir accéder aux **logins, cookies, autofills**, etc.

#### Logins, history, autofills :
- Récupère la **"Master Key"** (clé AES) dans le fichier `Local State`
- Ouvre les fichiers SQLite de Chrome : `Login Data`, `Web Data`, etc.
- Décrypte les champs chiffrés (mots de passe, numéros de carte, champs auto-complétés)
- Formate les données pour qu'elles soient **facilement lisibles**

#### Cookies :
- Utilise aussi la **Master Key** (depuis `Local State`)
- Ouvre la base `Cookies` (fichier SQLite)

- Pour les **cookies v10** :
  - Détection du préfixe `v10`
  - Déchiffrement via **AES-GCM** (avec IV, données, tag)
  - Reconstruction du cookie : nom, valeur, domaine…

- Pour les **cookies v20** (Chrome 118+) :
  - Détection du préfixe `v20`
  - Extraction du `nonce`, des données chiffrées et du `tag`
  - Déchiffrement avec **AES-GCM** + Master Key
  - Formatage final du cookie en clair

> [!WARNING]  
> Le **véritable danger**, ce sont les **cookies décryptés**.  
> Ils permettent à un attaquant d'accéder à vos **sessions en cours**,  
> sans jamais passer par vos mots de passe ni la double authentification (2FA).

Avec des cookies volés, un attaquant peut utiliser l'extension  
[Cookie Quick Manager](https://addons.mozilla.org/en-US/firefox/addon/cookie-quick-manager/)  
(disponible uniquement sur Firefox) pour **importer vos cookies volés**  
et **se connecter à vos comptes** (Instagram, Spotify, etc.)  
comme s’il s’agissait de votre propre session.

> [!TIP]  
> Pour **révoquer l'accès via les cookies volés**,  
> **changez simplement vos mots de passe**. Cela les rend expirés/inutilisables.

> [!WARNING]  
> Même si c'est réversible, pendant le temps où l'attaquant a accès :  
> il peut **lire vos messages privés**, **poster des contenus à votre place**,  
> et **surveiller vos activités**, **sans que vous ne soyez averti**.  
> Certaines plateformes peuvent détecter un accès suspect (IP/location)  
> et vous envoyer un mail d’alerte, mais **ce n’est pas systématique**.
