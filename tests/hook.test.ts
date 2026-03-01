import { hookTests } from './__fixtures__/hook';
import {
    snapCases,
    createConfigurationSuites,
    cliConsistencyCases,
} from './testUtils';

const suites = createConfigurationSuites('hooks', hookTests);

snapCases(suites);
cliConsistencyCases(suites);
