# `forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: e96606e3  
- **Timestamp**: 2025-10-10T02:16:00.662393+00:00  
- **Lines of Code**: 864

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 864 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tests in Source Directory


### Line 205: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 205)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use crate::idx::ft::analyzer::tests::{test_analyzer, test_analyzer_tokens};
	use crate::idx::ft::analyzer::tokenizer::Token;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 210: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 210)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_arabic_stemmer() {
		let input = "الكلاب تحب الجري في الحديقة، لكن كلبي الصغير يفضل النوم في سريره بدلاً من الجري";
		let output = vec![
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 233: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 233)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_danish_stemmer() {
		let input = "Hunde elsker at løbe i parken, men min lille hund foretrækker at sove i sin kurv frem for at løbe.";
		let output = vec![
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 276: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 276)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_dutch_stemmer() {
		let input = "Honden houden ervan om in het park te rennen, maar mijn kleine hond slaapt liever in zijn mand dan te rennen.";
		let output = vec![
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 299: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 299)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_english_stemmer() {
		let input = "Teachers are often teaching, but my favorite teacher prefers reading in her spare time rather than teaching.";
		let output = vec![
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 322: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 322)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_finnish_stemmer() {
		let input = "työ tekijäänsä kiittää";
		let output = ["työ", "tekij", "kiit"];
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 342: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 342)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_french_stemmer() {
		let input = "Les chiens adorent courir dans le parc, mais mon petit chien aime plutôt se blottir sur le canapé que de courir";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 365: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 365)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_german_stemmer() {
		let input = "Hunde lieben es, im Park zu laufen, aber mein kleiner Hund zieht es vor, auf dem Sofa zu schlafen, statt zu laufen.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 389: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 389)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_greek_stemmer() {
		let input = "Τα σκυλιά αγαπούν να τρέχουν στο πάρκο, αλλά ο μικρός μου σκύλος προτιμά να κοιμάται στο κρεβάτι του αντί να τρέχει.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 433: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 433)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_hungarian_stemmer() {
		let input = "A kutyák szeretnek futni a parkban, de az én kicsi kutyám inkább alszik a kosarában, mintsem fut.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 456: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 456)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_italian_stemmer() {
		let input = "I cani amano correre nel parco, ma il mio piccolo cane preferisce dormire nel suo cesto piuttosto che correre.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 479: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 479)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_norwegian_stemmer() {
		let input = "Hunder elsker å løpe i parken, men min lille hund foretrekker å sove i sengen sin heller enn å løpe.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 522: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 522)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_portuguese_stemmer() {
		let input = "Os cães adoram correr no parque, mas o meu pequeno cão prefere dormir na sua cama em vez de correr.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 545: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 545)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_romanian_stemmer() {
		let input = "Câinii adoră să alerge în parc, dar cățelul meu preferă să doarmă în coșul lui decât să alerge.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 568: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 568)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_russian_stemmer() {
		let input = "Собаки любят бегать в парке, но моя маленькая собака предпочитает спать в своей корзине, а не бегать.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 609: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 609)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_spanish_stemmer() {
		let input = "Los perros aman correr en el parque, pero mi pequeño perro prefiere dormir en su cama en lugar de correr.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 632: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 632)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_swedish_stemmer() {
		let input = "Hundar älskar att springa i parken, men min lilla hund föredrar att sova i sin säng istället för att springa.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 675: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 675)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_tamil_stemmer() {
		let input = "நாய்கள் பூங்காவில் ஓடுவதை விரும்புகின்றன, ஆனால் என் சிறிய நாய் அதன் படுகையில் தூங்குவதை விரும்புகின்றது, ஓட இல்லை.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 721: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 721)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_turkish_stemmer() {
		let input = "Köpekler parkta koşmayı sever, ama benim küçük köpeğim koşmaktansa yatağında uyumayı tercih eder.";
		let output = [
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 744: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 744)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_ngram() {
		test_analyzer(
			"ANALYZER test TOKENIZERS blank,class FILTERS lowercase,ngram(2,3);",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 757: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 757)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_ngram_tokens() {
		test_analyzer_tokens(
			"ANALYZER test TOKENIZERS blank,class FILTERS lowercase,ngram(2,3);",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 840: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 840)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_edgengram() {
		test_analyzer(
			"ANALYZER test TOKENIZERS blank,class FILTERS lowercase,edgengram(2,3);",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 850: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 850)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_lowercase_tokens() {
		test_analyzer_tokens(
			"ANALYZER test TOKENIZERS blank,class FILTERS lowercase",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 878: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/analyzer/filter.rs` (line 878)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn test_uppercase_tokens() {
		test_analyzer_tokens(
			"ANALYZER test TOKENIZERS blank,class FILTERS uppercase",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym