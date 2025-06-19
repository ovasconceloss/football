import { BaseError } from "./BaseError";

export class ConfigServerError extends BaseError {
    constructor(message: string) {
        super('ConfigError', 500, message, false);
    }
}

export class InternalServerError extends BaseError {
    constructor(message: string = 'Internal server error.') {
        super('InternalServerError', 500, message, false);
    }
}

export class NotFoundError extends BaseError {
    constructor(message: string = 'Resource not found.') {
        super('NotFoundError', 404, message, true);
    }
}

export class ValidationError extends BaseError {
    constructor(message: string = 'Invalid input data.') {
        super('ValidationError', 400, message, true);
    }
}

export class UnauthorizedError extends BaseError {
    constructor(message: string = 'Unauthorized') {
        super('UnauthorizedError', 401, message, true);
    }
}

export class ForbiddenError extends BaseError {
    constructor(message: string = 'Access denied.') {
        super('ForbiddenError', 403, message, true);
    }
}