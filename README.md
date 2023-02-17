# DIWL: Project for Polkadot Europe Edition Hacks

# Project name

The project's name is DIWL. It means Dynamic Individuation Word List.

# Target users

This dApp is mainly aimed at people with a certain level of language foundation.They already know some basic words and grammar, but their vocabulary is insufficient.

# What problem to solve

1. Build a **dynamic individuation word list**, collect new words by watching videos or browsing the web which they are interested in, translation and comment on the word they don't know, improve their foreign language skills gradually and naturally.
2. Let the foreign language beginner browse content on the Internet directly with the target language. Provides new ways to learn vocabulary and memorize words.
  
# Why use blockchain

Personalized word list is the user's private data, which is exactly the core problem to be solved by web3 technology. The data is stored on the block chain, and users can share or authorize their data, so as to further carry out some data analysis work and create more value.

# Core Functionality

Usually YouTube subtitles look like this:

![](images/Untitled.png)

When my English is not good, I don't know a certain word, for example:"treasury", I can't understand the meaning of this sentence.

So we need dictionaries.
Using a browser plug-in, we can automatically add the word's explanation to the end of the unfamiliar word, like this:

![](images/Untitled%201.png)

At the same time, we need to build a memorized list of words, and mark the words we already know. When we remember a word, the browser plug-in will not comment on it.

### Why Not

We can also use automatic translation, but machine translation has a lot of mistakes, and it's not good for us to improve our English.

# MVP

First, just annotate the content of one website(YouTube),  more support sites can be added later, like Medium Reddit etc.

Focusing on one use scenario allows the product to be developed quickly.

# The final completed demo
![](images/cc3.png)

# **How it work or implement**

1. The list of words is stone in the smart contract, based on Ink!

2. Import the dictionary data into the smart contract. The dictionary needs to be pre-processed to grade the words.

3. Use Tampermonkey script, load the front-end code, and provide wallet login function.

4. The front-end uses polkdot.js to cache the word data in the contract to the front-end, including the following: a. Users already know and don't need to be mark's words; b. A dictionary of high-frequency words. A functional interface that provides this capability is required in the contract.

5. When the code in the browser plug-in detects that YouTube is accessed, it first checks whether real-time subtitles are enabled. If they are not enabled, no action is taken.If enabled, confirm the language is English, start to work.

6. When the conditions are met, the plug-in code periodically detects the text content in subtitles (YouTube page DOM needs to be analyzed), and matches the current word with the previous cached data. The known word is not processed, and the unknown word is now queried in the high-frequency dictionary. After finding the word explanation, modify the subtitle content and add the word explanation.

7. Modify the DIV of subtitles on the web page so that each word can be clicked. After clicking, a pop-up window can be opened to query the explanation of the word and mark the word.

8. The browser plug-in provides a page to jump to remember words, which can be reviewed and re-marked. Since the signature is required to save data into the contract, the operation data is first saved in the cache, and then synchronized.

9. Provide word list merging function, smart contract also needs to add a support function.

# Teamwork

I am in the process of building a web3 development online team, which will be formed from the Polkdot Hackathon. There are currently have two members Zhaojie and Clive D. More people will join in the future.

The team runs in a way like Proof Of Work(POW). For example, I have an idea, I list all the things that need to be done,  and give each thing a score. If anyone completes it, he can get the score. If the project won the Hackathon, the prize was allocated according to the score, and if the task was not completed by anyone else, it was done by myself, and the score and the prize went to me.

The name of our team is: Sidado. It's a Chinese pronunciation from Siddhartha Gautama(Sakyamuni, founder of Buddhism), it means wisdom.

I don't want to do one-off projects, develop projects that must have real value, and develop projects that are interconnected.

If you have a good idea, I suggest you do the same as me. The online team is different from the real one.