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

Nous allons partir du principes que l'attaquant à utilisé un stealer open source. Maintenant qu'il à réussi à vous faire télécharger et executer son stealer, qu'elles sont les données qu'il a en sa possession ?  

https://github.com/doenerium6969/doenerium-fixed/blob/main/stub/stub.js#L272

https://github.com/PIKA-X-777/CStealer/blob/main/creal.py#L931

Voici une liste des informations qu'il risque de détenir:

 - les informations sur votre systeme (MAC, HWID, IP, KEY PRODUCT etc.)
 - Vos logins (web)
 - Votre historique (web)
 - Vos cookies (web)
 - Vos autofills (web)
 - Vos bookmarks (web)
 - Des fichiers spécifiques (password.txt, mdp.txt etc.)
 - Vos wallets
 - Votre session discord
 - etc.
