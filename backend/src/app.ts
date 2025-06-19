import Fastify from "fastify";

export async function applicationBuild() {
    try {
        const application = Fastify({
            logger: {
                level: process.env.LOG_LEVEL || 'info',
                transport: {
                    target: 'pino-pretty',
                    options: {
                        colorize: true,
                        translateTime: 'SYS:yyyy-mm-dd HH:MM:ss',
                        ignore: 'pid,hostname',
                    },
                },
            },
        });

        return application;
    } catch (err: unknown) {
        const error = err instanceof Error ? err : new Error(String(err));
        console.error('Failed during application build:', error.message);
        console.error(error);
        process.exit(1);
    }
};