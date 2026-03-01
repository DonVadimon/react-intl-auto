import { definitionTests } from './__fixtures__/definition';
import {
    snapCases,
    createConfigurationSuites,
    cliConsistencyCases,
} from './testUtils';

const suites = createConfigurationSuites('definition', definitionTests);

snapCases(suites);
cliConsistencyCases(suites);
