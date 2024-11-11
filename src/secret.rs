/// Wrapper type for the values that contains secrets, which attempts to limit
/// accidental exposure and ensure secrets are wiped from memory when droped.
/// (e.g. passwords, cryptographic keys, access tokens or other credentials)
/// 
/// Access to the secret inner value occurs through the [...]
/// `expose_secret()` method [...]
pub struct Secret<S>
    where S: Zerozise,
{
    /// Inner secret value
    inner_secret: S
}