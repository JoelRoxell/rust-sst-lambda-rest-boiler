import { StackContext, ApiGatewayV1Api } from "sst/constructs";

export function API({ stack }: StackContext) {
  const api = new ApiGatewayV1Api(stack, "rest-api", {
    defaults: {
      function: {
        runtime: "rust",
      },
    },
    routes: {
      "GET /": "packages/functions/target/lambda/get_index",
      "GET /{id}": "packages/functions/target/lambda/get_index_id",
    },
  });
  stack.addOutputs({
    ApiEndpoint: api.url,
  });
}
