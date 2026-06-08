import type { RouteRecord } from "vite-react-ssg";
import { AppLayout } from "@/components/AppLayout";
import Home from "./pages/Home";
import StrategicVision from "./pages/StrategicVision";
import SemanticModel from "./pages/SemanticModel";
import RequirementsCapabilities from "./pages/RequirementsCapabilities";
import Ontologies from "./pages/Ontologies";
import ModelingLanguage from "./pages/ModelingLanguage";
import UserGuide from "./pages/UserGuide";
import Advanced from "./pages/Advanced";
import Verifications from "./pages/Verifications";
import ImplementationCoverage from "./pages/ImplementationCoverage";
import Submodels from "./pages/Submodels";
import Integrations from "./pages/Integrations";
import McpServer from "./pages/McpServer";
import CodingAssistants from "./pages/CodingAssistants";
import NotFound from "./pages/NotFound";

export const routes: RouteRecord[] = [
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <Home /> },
      { path: "strategic-vision", element: <StrategicVision /> },
      { path: "semantic-model", element: <SemanticModel /> },
      {
        path: "requirements-capabilities",
        element: <RequirementsCapabilities />,
      },
      { path: "ontologies", element: <Ontologies /> },
      { path: "modeling-language", element: <ModelingLanguage /> },
      { path: "user-guide", element: <UserGuide /> },
      { path: "advanced", element: <Advanced /> },
      { path: "verifications", element: <Verifications /> },
      { path: "implementation-coverage", element: <ImplementationCoverage /> },
      { path: "submodels", element: <Submodels /> },
      { path: "integrations", element: <Integrations /> },
      { path: "mcp-server", element: <McpServer /> },
      { path: "coding-assistants", element: <CodingAssistants /> },
    ],
  },
  { path: "*", element: <NotFound /> },
];
