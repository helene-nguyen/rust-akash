use anyhow::{ Context, Result };
use serde::{ Deserialize, Serialize };
use std::collections::BTreeMap;
use std::path::{ PathBuf };
use tracing::{ info, debug };

/// JSON schema: {aliases: {alias_name: command, ...}}
/// BTreeMap is used to maintain sorted order of aliases for consistent display and testing.
#[derive(Debug, Serialize, Deserialize)]
pub struct AliasStore {
    pub aliases: BTreeMap<String, String>,
}

impl AliasStore {
    /// Create a new empty store.
    pub fn new_store() -> Self {
        Self {
            aliases: BTreeMap::new(),
        }
    }

    /// Path to the JSON file where aliases are stored.
    /// Store to: ~/.akash/aliases.json
    pub fn store_path(custom_path: Option<&PathBuf>) -> Result<PathBuf> {
        if let Some(path) = custom_path {
            debug!("Using custom aliases path: {}", path.display());
            return Ok(path.clone());
        }
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
        let path = home.join(".akash").join("aliases.json");
        debug!("Alias store path: {}", path.display());
        Ok(path)
    }

    /// Load the store from disk, or create a new one if it doesn't exist.
    pub fn store_load(custom_path: Option<&PathBuf>) -> Result<Self> {
        let path = Self::store_path(custom_path)?;

        // If the file doesn't exist, we start with an empty store. This allows us to create aliases without needing a pre-existing file.
        if !path.exists() {
            debug!("No aliases file found at {}, starting fresh", path.display());
            return Ok(Self::new_store());
        }

        // .with_context() adds the file path to the error message if reading fails, making it easier to debug issues related to file access.
        let content = std::fs
            ::read_to_string(&path)
            .with_context(|| format!("Failed to read alias store from {}", path.display()))?;

        let store: Self = serde_json
            ::from_str(&content)
            .with_context(|| format!("Failed to parse alias store JSON from {}", path.display()))?;

        debug!("Loaded alias store with {} aliases", store.aliases.len());
        Ok(store)
    }

    /// Save the store to disk, creating the directory if it doesn't exist.
    pub fn store_save(&self, custom_path: Option<&PathBuf>) -> Result<()> {
        let path = Self::store_path(custom_path)?;

        if let Some(parent) = path.parent() {
            std::fs
                ::create_dir_all(&parent)
                .with_context(||
                    format!("Failed to create directory for alias store at {}", parent.display())
                )?;
        }

        let content = serde_json
            ::to_string_pretty(self)
            .context("Failed to serialize alias store to JSON")?;

        std::fs
            ::write(&path, content)
            .with_context(|| format!("Failed to write alias store to {}", path.display()))?;

        // Debug displays: {"level":"DEBUG", "message":"Saved alias store with N aliases", "path":"/home/user/.akash/aliases.json"}
        debug!("Saved in alias store with {} aliases", self.aliases.len());
        Ok(())
    }

    /// Add an alias to the store and save it.
    pub fn add_alias(&mut self, alias_name: String, command: String) -> bool {
        let is_new = !self.aliases.contains_key(&alias_name);

        if is_new {
            debug!("Adding new alias: {} -> {}", alias_name, command);
        } else {
            debug!("Updating existing alias: {} -> {}", alias_name, command);
        }

        self.aliases.insert(alias_name, command);
        is_new
    }

    /// Remove an alias. Returns true if found and removed.
    pub fn remove_alias(&mut self, alias_name: &str) -> bool {
        let removed = self.aliases.remove(alias_name).is_some();
        if removed {
            info!("Removed alias: {}", alias_name);
        } else {
            info!("Alias not found for removal: {}", alias_name);
        }
        removed
    }

    /// Reference to all aliases.
    pub fn list_aliases(&self) -> &BTreeMap<String, String> {
        info!("Listing {} aliases", self.aliases.len());
        &self.aliases
    }

    /// Check if an alias exists.
    pub fn has_key(&self, alias_name: &str) -> bool {
        let exists = self.aliases.contains_key(alias_name);
        info!("Checking alias '{}': {}", alias_name, if exists {
            "Alias found"
        } else {
            "Alias not found"
        });
        exists
    }

    // Validate an alias name: only alphanumeric, _ and - allowed.
    // This ensures that alias names are simple and won't cause issues in shell commands.
    pub fn validate_alias_name(alias_name: &str) -> Result<()> {
        if alias_name.is_empty() {
            debug!("Alias name validation failed: alias_name is empty");
            anyhow::bail!("Alias name cannot be empty!");
        }

        if !alias_name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            debug!("Alias name validation failed: '{}' contains invalid characters", alias_name);
            anyhow::bail!(
                "Alias name can only contain alphanumeric characters, underscores, or hyphens!"
            );
        }
        debug!("Alias name '{}' is valid", alias_name);
        Ok(())
    }
}

/// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    // --- new_store ---

    #[test]
    fn given_nothing_when_creating_new_store_then_it_is_empty() {
        // Given
        // (no preconditions)

        // When
        let store = AliasStore::new_store();

        // Then
        assert!(store.aliases.is_empty());
    }

    // --- add_alias ---

    #[test]
    fn given_empty_store_when_adding_alias_then_returns_true() {
        // Given
        let mut store = AliasStore::new_store();

        // When
        let result = store.add_alias("gs".into(), "git status".into());

        // Then
        assert!(result);
    }

    #[test]
    fn given_existing_alias_when_adding_same_name_then_returns_false() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("gs".into(), "git status".into());

        // When
        let result = store.add_alias("gs".into(), "git stash".into());

        // Then
        assert!(!result);
    }

    #[test]
    fn given_existing_alias_when_adding_same_name_then_command_is_updated() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("gs".into(), "git status".into());

        // When
        store.add_alias("gs".into(), "git stash".into());

        // Then
        assert_eq!(store.aliases["gs"], "git stash");
    }

    // --- remove_alias ---

    #[test]
    fn given_existing_alias_when_removing_it_then_returns_true_and_store_is_empty() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("gs".into(), "git status".into());

        // When
        let result = store.remove_alias("gs");

        // Then
        assert!(result);
        assert!(store.aliases.is_empty());
    }

    #[test]
    fn given_empty_store_when_removing_alias_then_returns_false() {
        // Given
        let mut store = AliasStore::new_store();

        // When
        let result = store.remove_alias("nonexistent");

        // Then
        assert!(!result);
    }

    // --- has_key ---

    #[test]
    fn given_existing_alias_when_checking_has_key_then_returns_true() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("gs".into(), "git status".into());

        // When
        let result = store.has_key("gs");

        // Then
        assert!(result);
    }

    #[test]
    fn given_empty_store_when_checking_has_key_then_returns_false() {
        // Given
        let store = AliasStore::new_store();

        // When
        let result = store.has_key("nope");

        // Then
        assert!(!result);
    }

    // --- list_aliases ---

    #[test]
    fn given_two_aliases_when_listing_then_returns_both_with_correct_values() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("a".into(), "alpha".into());
        store.add_alias("b".into(), "bravo".into());

        // When
        let list = store.list_aliases();

        // Then
        assert_eq!(list.len(), 2);
        assert_eq!(list["a"], "alpha");
        assert_eq!(list["b"], "bravo");
    }

    #[test]
    fn given_unordered_aliases_when_listing_then_keys_are_sorted() {
        // Given
        let mut store = AliasStore::new_store();
        store.add_alias("z".into(), "zulu".into());
        store.add_alias("a".into(), "alpha".into());
        store.add_alias("m".into(), "mike".into());

        // When
        let keys: Vec<&String> = store.list_aliases().keys().collect();

        // Then
        assert_eq!(keys, vec!["a", "m", "z"]);
    }

    // --- validate_alias_name ---

    #[test]
    fn given_valid_names_when_validating_then_all_pass() {
        // Given / When / Then
        assert!(AliasStore::validate_alias_name("gs").is_ok());
        assert!(AliasStore::validate_alias_name("my-alias").is_ok());
        assert!(AliasStore::validate_alias_name("my_alias").is_ok());
        assert!(AliasStore::validate_alias_name("alias123").is_ok());
    }

    #[test]
    fn given_empty_name_when_validating_then_returns_error() {
        // Given
        let name = "";

        // When
        let result = AliasStore::validate_alias_name(name);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_names_with_invalid_chars_when_validating_then_returns_error() {
        // Given / When / Then
        assert!(AliasStore::validate_alias_name("has space").is_err());
        assert!(AliasStore::validate_alias_name("has!bang").is_err());
        assert!(AliasStore::validate_alias_name("a/b").is_err());
    }
}
