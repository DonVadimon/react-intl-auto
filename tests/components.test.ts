import { componentsTests } from './__fixtures__/components';
import {
    snapCases,
    createConfigurationSuites,
    cliConsistencyCases,
} from './testUtils';

const suites = createConfigurationSuites('components', componentsTests);

snapCases(suites);
cliConsistencyCases(suites);
