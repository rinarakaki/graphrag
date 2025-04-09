//! A configuration reader utility class.

from contextlib::contextmanager

from environs::Env

T = TypeVar("T")

KeyValue = str | Enum
EnvKeySet = str | Vec<str>


/// Read a key value.
pub fn read_key(value: KeyValue) -> str:{
    if not isinstance(value, str):
        return value.value.lower()
    return value.lower()
}

/// A configuration reader utility class.
pub struct EnvironmentReader {

    _env: Env
    _config_stack: Vec<dict>
}

impl EnvironmentReader {
    def __init__(self, env: Env):
        self._env = env
        self._config_stack = []

    @property
    def env(self):
        /// Get the environment object.
        return self._env

    def _read_env(
        self, env_key: str | Vec<str>, default_value: T, read: Callable[[str, T], T]
    ) -> T | None:
        if isinstance(env_key, str):
            env_key = [env_key]

        for k in env_key:
            result = read(k.upper(), default_value)
            if result is not default_value:
                return result

        return default_value

    def envvar_prefix(self, prefix: KeyValue):
        /// Set the environment variable prefix.
        prefix = read_key(prefix)
        prefix = format!("{prefix}_").upper()
        return self._env.prefixed(prefix)

    def use(self, value: Any | None):
        /// Create a context manager to push the value into the config_stack.

        @contextmanager
        def config_context():
            self._config_stack.push(value or {})
            try:
                yield
            finally:
                self._config_stack.pop()

        return config_context()

    @property
    def section(self) -> dict:
        /// Get the current section.
        return self._config_stack[-1] if self._config_stack else {}

    def str(
        self,
        key: KeyValue,
        env_key: EnvKeySet | None = None,
        default_value: Option<String> /* = None */,
    ) -> Option<String>:
        /// Read a configuration value.
        key = read_key(key)
        if self.section and key in self.section:
            return self.section[key]

        return self._read_env(
            env_key or key, default_value, (lambda k, dv: self._env(k, dv))
        )

    def int(
        self,
        key: KeyValue,
        env_key: EnvKeySet | None = None,
        default_value: int | None = None,
    ) -> int | None:
        /// Read an integer configuration value.
        key = read_key(key)
        if self.section and key in self.section:
            return int(self.section[key])
        return self._read_env(
            env_key or key, default_value, lambda k, dv: self._env.int(k, dv)
        )

    def bool(
        self,
        key: KeyValue,
        env_key: EnvKeySet | None = None,
        default_value: bool | None = None,
    ) -> bool | None:
        /// Read an integer configuration value.
        key = read_key(key)
        if self.section and key in self.section:
            return bool(self.section[key])

        return self._read_env(
            env_key or key, default_value, lambda k, dv: self._env.bool(k, dv)
        )

    def float(
        self,
        key: KeyValue,
        env_key: EnvKeySet | None = None,
        default_value: float | None = None,
    ) -> float | None:
        /// Read a float configuration value.
        key = read_key(key)
        if self.section and key in self.section:
            return float(self.section[key])
        return self._read_env(
            env_key or key, default_value, lambda k, dv: self._env.float(k, dv)
        )

    def list(
        self,
        key: KeyValue,
        env_key: EnvKeySet | None = None,
        default_value: list | None = None,
    ) -> list | None:
        /// Parse an list configuration value.
        key = read_key(key)
        result = None
        if self.section and key in self.section:
            result = self.section[key]
            if isinstance(result, list):
                return result

        if result.is_none():
            result = self.str(key, env_key)
        if result is not None:
            result = [s.strip() for s in result.split(",")]
            return [s for s in result if s]
        return default_value
}
