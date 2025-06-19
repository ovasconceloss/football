import { applicationBuild } from "./app";
import { getServerConfig } from "./config";

const serverConfig = getServerConfig();

export async function serverStart() {
    const application = await applicationBuild();

    try {
        await application.listen({ port: serverConfig.server.port, host: '0.0.0.0' });
        application.log.info(`Server listening on ${serverConfig.server.port}`);
    } catch (err: unknown) {
        const error = err instanceof Error ? err : new Error(String(err));
        application.log.error(`Failed to start the Fastify server: ${error.message}`);
        application.log.error(error);
        process.exit(1);
    }
};