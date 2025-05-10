# 🇬🇧 English Version
The goal of this document about what a "stealer" is, is to help you understand how **dangerous** it can be and that they can be found **everywhere**.

---

In short, a "stealer" is a malicious program designed to **steal sensitive information** from us. Attackers may want your information for several reasons:
 - **Access to your email accounts** (to resell them or repeat the attack by impersonating you using your email)
 - **Admin access to your company account** (to access sensitive folders/files that can then be used for blackmail or simply resold)
 - **Information about your banking data** (to then perform phishing and try to steal your money)

In other cases, it can directly be a **ransom demand**, otherwise they will expose your data to, for example, all your contacts or on sites specialized in **"doxing"**.

> [!CAUTION]  
> **Never pay the ransom** (/ never give in to their demands).  
> The only thing the attacker will do after receiving the payment is to **leak your data** and disappear.  
> Here are useful links to limit the damage:  
> - [**WebCleaner**](https://www.webcleaner.fr/)
> - [**Incogni**](https://nordvpn.com/fr/incogni/)
> 
> Platforms most likely to host your "dox":
> - [**Doxbin**](https://www.doxbin.com/)
> - [**Zerobin**](https://zerobin.net/)
> - [**Pastebin**](https://pastebin.com/)

---

Now, let's see **everything a stealer does**, **where it gets the information from**, and **how the hacker uses it**.

You should know that **not all stealers are the same**, it all depends on several factors:

- **Did the attacker code it themselves** or **just buy it**, or even **grabbed the first open-source stealer available on GitHub**?
- **Has the attacker already seen** (via a photo or video posted on a social network, for example) **your desktop**, the different **files/folders it contained**, and **the web browsers you use**?
- **Your operating system** (*Windows, Linux, macOS*)

All of this matters because, take for example:

- **Did the attacker code it themselves** or **use an existing stealer**?

If the attacker coded it themselves, and on top of that, they have already seen:
- **Your desktop**
- **Your web browsers**
- **Your applications**, etc.

And on top of that, they **know your operating system**, they can very well develop **a stealer compatible with it**, and **look in the right place** by optimizing their code to recover **your logins** and **browser cookies**.

## How does it work?

Let's assume the attacker used an **open-source stealer**.  
Now that they managed to get you to download and execute their stealer, **what data do they have in their possession**?

Here is a list of **information they might hold**:

- **System information** (MAC, HWID, IP, PRODUCT KEY, etc.)
- **Your logins** (web)
- **Your history** (web)
- **Your cookies** (web)
- **Your autofills** (web)
- **Your bookmarks** (web)
- **Specific files** (`password.txt`, `mdp.txt`, etc.)
- **Your wallets**
- **Your Discord session**
- etc.

> [!NOTE]  
> You can visit these two GitHubs to get a better view  
> of what you can get with an open-source stealer:
> - [CStealer](https://github.com/PIKA-X-777/CStealer)
> - [Doenerium](https://github.com/doenerium6969/doenerium-fixed)
>
> And here to understand **where it retrieves them from**:
> - [CStealer Path](https://github.com/PIKA-X-777/CStealer/blob/main/creal.py#L931)
> - [Doenerium Path](https://github.com/doenerium6969/doenerium-fixed/blob/main/stub/stub.js#L272)

> [!TIP]  
> To understand **how a stealer is built**,  
> check out the GitHub of [Lawxsz](https://github.com/Lawxsz/make-u-own-stealer/)  
> (very useful if you want to learn more about how they work).

For those who struggle with Python, here is a **simplified explanation** of how it works.

Stealers simply:

- **Kill annoying tasks** (active browsers that block access to data)
- **Search for targeted files**
- **Decrypt** the data (if necessary)
- **Send them** to the attacker (webhook, FTP, etc.)

#### To "kill annoying tasks":
The program **closes browsers** (Chrome, Edge, Firefox, etc.)  
to then access **logins, cookies, autofills**, etc.

#### Logins, history, autofills:
- Retrieves the **"Master Key"** (AES key) from the `Local State` file
- Opens Chrome SQLite files: `Login Data`, `Web Data`, etc.
- Decrypts encrypted fields (passwords, card numbers, auto-completed fields)
- Formats the data to make it **easily readable**

#### Cookies:
- Also uses the **Master Key** (from `Local State`)
- Opens the `Cookies` database (SQLite file)

- For **v10 cookies**:
  - Detects the `v10` prefix
  - Decrypts via **AES-GCM** (with IV, data, tag)
  - Reconstructs the cookie: name, value, domain…

- For **v20 cookies** (Chrome 118+):
  - Detects the `v20` prefix
  - Extracts `nonce`, encrypted data, and `tag`
  - Decrypts using **AES-GCM** + Master Key
  - Final formatting of the cookie in plain text

> [!WARNING]  
> The **real danger** is the **decrypted cookies**.  
> They allow an attacker to access your **active sessions**,  
> without ever needing your passwords or two-factor authentication (2FA).

With stolen cookies, an attacker can use the  
[Cookie Quick Manager](https://addons.mozilla.org/en-US/firefox/addon/cookie-quick-manager/) extension  
(available only on Firefox) to **import your stolen cookies**  
and **log into your accounts** (Instagram, Spotify, etc.)  
as if it were your own session.

> [!TIP]  
> To **revoke access through stolen cookies**,  
> **simply change your passwords**. This makes them expired/unusable.

> [!WARNING]  
> Even if it's reversible, during the time the attacker has access:  
> they can **read your private messages**, **post content as you**,  
> and **monitor your activities**, **without you being notified**.  
> Some platforms may detect suspicious access (IP/location)  
> and send you an alert email, but **this is not guaranteed**.
