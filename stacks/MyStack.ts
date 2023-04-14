import { StackContext, ApiGatewayV1Api } from "sst/constructs";

export function API({ stack }: StackContext) {
  const api = new ApiGatewayV1Api(stack, "rest-api", {
    defaults: {
      function: {
        runtime: "rust",
      },
    },
    routes: {
      // This path must map to the binaries that are produced after build.
      // Outlined in ~/packages/functions/Cargo.toml
      "GET /": "target/lambda/get_index",
      "GET /{id}": "target/lambda/get_index_id",
    },
  });
  stack.addOutputs({
    ApiEndpoint: api.url,
  });
}
