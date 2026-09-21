"""Explicit errors returned by deterministic workflow gates."""


class ForhemitError(Exception):
    """Base domain error."""


class AuthorizationError(ForhemitError):
    """The actor is not allowed to perform the command."""


class InvariantError(ForhemitError):
    """A deterministic domain or contract invariant failed."""


class ConflictError(ForhemitError):
    """The command conflicts with current immutable state."""


class NotFoundError(ForhemitError):
    """A referenced object or version does not exist."""