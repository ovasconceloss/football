import "dotenv/config";
import { ConfigServerError } from "../utils/errors/ApplicationError";

export function getServerConfig() {
    const serverPort = Number(process.env.PORT || '8080');

    if (isNaN(serverPort) || serverPort <= 0 || serverPort >= 65536)
        throw new ConfigServerError(`Invalid PORT value: "${process.env.PORT}"`);

    return {
        server: {
            port: serverPort,
        },
    };
};