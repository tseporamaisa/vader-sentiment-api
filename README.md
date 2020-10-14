# VADER-Sentiment-Analysis-API



VADER (Valence Aware Dictionary and sEntiment Reasoner) is a lexicon and rule-based sentiment analysis tool that is *specifically attuned to sentiments expressed in social media*. It is fully open-sourced under the [MIT License](http://choosealicense.com/). [The original module](https://github.com/cjhutto/vaderSentiment) was written in Python and [ported to Rust](https://github.com/ckw017/vader-sentiment-rust). This is a web server exposing a REST Interface for the Rust port (including [pull requests](https://github.com/ckw017/vader-sentiment-rust/pulls) by ambaxter).

# Use Cases
	* examples of typical use cases for sentiment analysis, including proper handling of sentences with:

		- typical negations (e.g., "not good")
		- use of contractions as negations (e.g., "wasn't very good")
		- conventional use of punctuation to signal increased sentiment intensity (e.g., "Good!!!")
		- conventional use of word-shape to signal emphasis (e.g., using ALL CAPS for words/phrases)
		- using degree modifiers to alter sentiment intensity (e.g., intensity boosters such as "very" and intensity dampeners such as "kind of")
		- understanding many sentiment-laden slang words (e.g., 'sux')
		- understanding many sentiment-laden slang words as modifiers such as 'uber' or 'friggin' or 'kinda'
		- understanding many sentiment-laden emoticons such as :) and :D
		- translating utf-8 encoded emojis such as 💘 and 💋 and 😁
		- understanding sentiment-laden initialisms and acronyms (for example: 'lol')

	* more examples of tricky sentences that confuse other sentiment analysis tools
	* example for how VADER can work in conjunction with NLTK to do sentiment analysis on longer texts...i.e., decomposing paragraphs, articles/reports/publications, or novels into sentence-level analyses
	* examples of a concept for assessing the sentiment of images, video, or other tagged multimedia content
	* if you have access to the Internet, the demo has an example of how VADER can work with analyzing sentiment of texts in other languages (non-English text sentences).

# Usage

## With docker-compose
``` bash
docker-compose up -d

curl -H "Content-Type: application/json" -X POST -d '{"text":["VADER is smart, handsome, and funny.","At least it is not a horrible book."]}' http://127.0.0.1:8080/get_sentiment

```    
### Output
``` json
[
    {
        "text": "VADER is smart, handsome, and funny.",
        "neg": 0.0,
        "neu": 0.2542372881355932,
        "pos": 0.7457627118644068,
        "compound": 0.8316320352807864
    },
    {
        "text": "At least it is not a horrible book.",
        "neg": 0.0,
        "neu": 0.6779661016949153,
        "pos": 0.3220338983050848,
        "compound": 0.43102002306105164
    }
]
```   
## Natively    

``` rust
cargo run
```    

``` bash
curl -H "Content-Type: application/json" -X POST -d '{"text":["VADER is smart, handsome, and funny.","At least it is not a horrible book."]}' http://127.0.0.1:8080/get_sentiment
```      

   
> **⚠ WARNING: No security.**  
> There are no security measures taken here so it is not a good idea to expose the server directly to the internet. If used as is, expose only to trusted clients inside trusted environments. 
