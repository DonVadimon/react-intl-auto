import { injectionTests } from './__fixtures__/injection';
import {
    snapCases,
    createConfigurationSuites,
    cliConsistencyCases,
} from './testUtils';

const suites = createConfigurationSuites('injection', injectionTests);

snapCases(suites);
cliConsistencyCases(suites);
