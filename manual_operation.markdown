PowerShell 7.2.1
Copyright (c) Microsoft Corporation.

https://aka.ms/powershell
Type 'help' to get help.

PS C:\Users\kushal\src\beginningmachinelearning> npm --version
8.3.1
PS C:\Users\kushal\src\beginningmachinelearning> npm install --global yarn

added 1 package, and audited 2 packages in 2s

found 0 vulnerabilities
npm notice
npm notice New minor version of npm available! 8.3.1 -> 8.5.4
npm notice Changelog: https://github.com/npm/cli/releases/tag/v8.5.4
npm notice Run npm install -g npm@8.5.4 to update!
npm notice
PS C:\Users\kushal\src\beginningmachinelearning> date; yarn; date;

Monday, March 14, 2022 8:32:29 AM
yarn install v1.22.17
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
warning "@zeit/ng-deploy > now-client > @zeit/fetch@5.1.0" has unmet peer dependency "@types/node-fetch@2".
warning " > codelyzer@6.0.0" has incorrect peer dependency "@angular/compiler@>=2.3.1 <11.0.0 || >9.0.0-beta <11.0.0 || >9.1.0-beta <11.0.0 || >9.2.0-beta <11.0.0".
warning " > codelyzer@6.0.0" has incorrect peer dependency "@angular/core@>=2.3.1 <11.0.0 || >9.0.0-beta <11.0.0 || >9.1.0-beta <11.0.0 || >9.2.0-beta <11.0.0".
warning " > karma-jasmine-html-reporter@1.6.0" has incorrect peer dependency "jasmine-core@>=3.7.1".
[4/4] Building fresh packages...
Done in 107.92s.
Monday, March 14, 2022 8:34:17 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; ng update; date

Monday, March 14, 2022 8:54:36 AM
ng: The term 'ng' is not recognized as a name of a cmdlet, function, script file, or executable program.
Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
Monday, March 14, 2022 8:54:37 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date

Monday, March 14, 2022 8:54:43 AM
yarn run v1.22.17
$ ng update
The installed local Angular CLI version is older than the latest stable version.
Installing a temporary version to perform the update.
Installing packages for tooling via yarn.
Installed packages for tooling via yarn.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json, there are some packages to update:

      Name                               Version                  Command to update
     --------------------------------------------------------------------------------
      @angular/cli                       11.2.12 -> 12.2.9        ng update @angular/cli@12
      @angular/core                      11.2.13 -> 12.2.9        ng update @angular/core@12

    There might be additional packages which don't provide 'ng update' capabilities that are outdated.
    You can update the additional packages by running the update command of your package manager.
Done in 46.47s.
Monday, March 14, 2022 8:55:30 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date; yarn run ng update @angular/cli @angular/core; date;

Monday, March 14, 2022 8:56:21 AM
yarn run v1.22.17
$ ng update
The installed local Angular CLI version is older than the latest stable version.
Installing a temporary version to perform the update.
Installing packages for tooling via yarn.
Installed packages for tooling via yarn.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json, there are some packages to update:

      Name                               Version                  Command to update
     --------------------------------------------------------------------------------
      @angular/cli                       11.2.12 -> 12.2.9        ng update @angular/cli@12
      @angular/core                      11.2.13 -> 12.2.9        ng update @angular/core@12

    There might be additional packages which don't provide 'ng update' capabilities that are outdated.
    You can update the additional packages by running the update command of your package manager.
Done in 25.10s.
Monday, March 14, 2022 8:56:47 AM
yarn run v1.22.17
$ ng update @angular/cli @angular/core
The installed local Angular CLI version is older than the latest stable version.
Installing a temporary version to perform the update.
Installing packages for tooling via yarn.
Installed packages for tooling via yarn.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
Fetching dependency metadata from registry...
Updating multiple major versions of '@angular/cli' at once is not supported. Please migrate each major version individually.
Run 'ng update @angular/cli@12' in your workspace directory to update to latest '12.x' version of '@angular/cli'.

For more information about the update process, see https://update.angular.io/?v=11.0-12.0
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
Monday, March 14, 2022 8:57:11 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date; yarn run ng update @angular/cli@12 @angular/core@12; date;

Monday, March 14, 2022 8:58:39 AM
yarn run v1.22.17
$ ng update
The installed local Angular CLI version is older than the latest stable version.
Installing a temporary version to perform the update.
Installing packages for tooling via yarn.
Installed packages for tooling via yarn.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json, there are some packages to update:

      Name                               Version                  Command to update
     --------------------------------------------------------------------------------
      @angular/cli                       11.2.12 -> 12.2.9        ng update @angular/cli@12
      @angular/core                      11.2.13 -> 12.2.9        ng update @angular/core@12

    There might be additional packages which don't provide 'ng update' capabilities that are outdated.
    You can update the additional packages by running the update command of your package manager.
Done in 24.77s.
Monday, March 14, 2022 8:59:04 AM
yarn run v1.22.17
$ ng update @angular/cli@12 @angular/core@12
The installed local Angular CLI version is older than the latest stable version.
Installing a temporary version to perform the update.
Installing packages for tooling via yarn.
Installed packages for tooling via yarn.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
Fetching dependency metadata from registry...
    Updating package.json with dependency @angular-devkit/build-angular @ "12.2.16" (was "0.1102.12")...
    Updating package.json with dependency @angular/cli @ "12.2.16" (was "11.2.12")...
    Updating package.json with dependency @angular/compiler-cli @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/language-service @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency typescript @ "4.3.5" (was "4.1.5")...
    Updating package.json with dependency @angular/animations @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/common @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/compiler @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/core @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/forms @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/platform-browser @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/platform-browser-dynamic @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency @angular/router @ "12.2.16" (was "11.2.13")...
    Updating package.json with dependency zone.js @ "0.11.5" (was "0.10.3")...
  UPDATE package.json (1431 bytes)
✔ Packages successfully installed.
** Executing migrations of package '@angular/cli' **

> Remove deprecated options from 'angular.json' that are no longer present in v12.
    "deploy" target in "beginningmachinelearning" project is using a third-party builder.
    You may need to adjust the options to retain the existing behavior.
    For more information, see the breaking changes section within the release notes: https://github.com/angular/angular-cli/releases/tag/v12.0.0
  UPDATE angular.json (4234 bytes)
  Migration completed.

> Update 'zone.js' to version 0.11.x.
  Read more about this here: https://github.com/angular/angular/blob/master/packages/zone.js/CHANGELOG.md#breaking-changes-since-zonejs-v0111
  UPDATE package.json (1431 bytes)
  UPDATE src/polyfills.ts (2891 bytes)
  UPDATE src/test.ts (652 bytes)
  UPDATE src/environments/environment.ts (681 bytes)
✔ Packages installed successfully.
  Migration completed.

> Remove 'emitDecoratorMetadata' TypeScript compiler option.
  Decorator metadata is no longer needed by Angular.
  Read more about this here: https://www.typescriptlang.org/docs/handbook/decorators.html#metadata
    Skipping migration as the workspace uses third-party builders which may require "emitDecoratorMetadata" TypeScript compiler option.
  Migration completed.

> Lazy loading syntax migration.
  Update lazy loading string syntax to use dynamic imports.
  Migration completed.

> Remove deprecated ViewEngine-based i18n build and extract options.
  Options present in the configuration will be converted to use non-deprecated options.
  Migration completed.

> Updates Web Worker consumer usage to use the new syntax supported directly by Webpack 5.
  Migration completed.

> Remove invalid 'skipTests' option in '@schematics/angular:module' Angular schematic options.
  Migration completed.

> Replace the deprecated '--prod' in package.json scripts.
  UPDATE package.json (1431 bytes)
  Migration completed.

** Executing migrations of package '@angular/core' **

> In Angular version 12, the type of ActivatedRouteSnapshot.fragment is nullable.
  This migration automatically adds non-null assertions to it.
  Migration completed.

> `XhrFactory` has been moved from `@angular/common/http` to `@angular/common`.
  Migration completed.

> Automatically migrates shadow-piercing selector from `/deep/` to the recommended alternative `::ng-deep`.
  Migration completed.

Done in 167.41s.
Monday, March 14, 2022 9:01:51 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date; yarn run ng update @angular/cli @angular/core; date;      

Monday, March 14, 2022 9:44:40 AM
yarn run v1.22.17
$ ng update
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json, there are some packages to update:

      Name                               Version                  Command to update
     --------------------------------------------------------------------------------
      @angular/cli                       12.2.16 -> 13.2.6        ng update @angular/cli
      @angular/core                      12.2.16 -> 13.2.6        ng update @angular/core

    There might be additional packages which don't provide 'ng update' capabilities that are outdated.
    You can update the additional packages by running the update command of your package manager.
Done in 11.63s.
Monday, March 14, 2022 9:44:52 AM
yarn run v1.22.17
$ ng update @angular/cli @angular/core
The installed Angular CLI version is outdated.
Installing a temporary Angular CLI versioned 13.2.6 to perform the update.
✔ Package successfully installed.
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
Fetching dependency metadata from registry...
    Updating package.json with dependency @angular-devkit/build-angular @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/cli @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/compiler-cli @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/language-service @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency typescript @ "4.5.5" (was "4.3.5")...
    Updating package.json with dependency @angular/animations @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/common @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/compiler @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/core @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/forms @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/platform-browser @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/platform-browser-dynamic @ "13.2.6" (was "12.2.16")...
    Updating package.json with dependency @angular/router @ "13.2.6" (was "12.2.16")...
  UPDATE package.json (1419 bytes)
✔ Packages successfully installed.
** Executing migrations of package '@angular/cli' **

> Remove polyfills required only for Internet Explorer which is no longer supported.
  UPDATE src/polyfills.ts (2396 bytes)
  Migration completed.

> Remove no longer valid Angular schematic options from `angular.json`.
  Migration completed.

> Remove deprecated options from 'angular.json' that are no longer present in v13.
  UPDATE angular.json (3876 bytes)
  Migration completed.

> Updating '.gitignore' to include '.angular/cache'.
  UPDATE .gitignore (693 bytes)
  Migration completed.

> Update library projects to be published in partial mode and removed deprecated options from ng-packagr configuration.
  Migration completed.

** Executing migrations of package '@angular/core' **

> Migrates `[routerLink]=""` in templates to `[routerLink]="[]"` because these links are likely intended to route to the current page with updated fragment/query params.
  Migration completed.

> In Angular version 13, the `teardown` flag in `TestBed` will be enabled by default.
  This migration automatically opts out existing apps from the new teardown behavior.
  UPDATE src/test.ts (701 bytes)
  Migration completed.

> As of Angular version 13, `entryComponents` are no longer necessary.
  Migration completed.

Done in 102.47s.
Monday, March 14, 2022 9:46:35 AM

PS C:\Users\kushal\src\beginningmachinelearning> git pull --rebase --strategy-option=ours
Successfully rebased and updated refs/heads/master.
PS C:\Users\kushal\src\beginningmachinelearning> git push
Enumerating objects: 29, done.
Counting objects: 100% (29/29), done.
Delta compression using up to 8 threads
Compressing objects: 100% (19/19), done.
Writing objects: 100% (19/19), 94.88 KiB | 2.26 MiB/s, done.
Total 19 (delta 17), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (17/17), completed with 9 local objects.
remote: 
remote: GitHub found 54 vulnerabilities on beginningmachinelearning/beginningmachinelearning's default branch (1 critical, 27 high, 23 moderate, 3 low). To find out more, visit:
remote:      https://github.com/beginningmachinelearning/beginningmachinelearning/security/dependabot
remote:
To github.com:beginningmachinelearning/beginningmachinelearning.git
   0dffad103..b51bf450b  master -> master
PS C:\Users\kushal\src\beginningmachinelearning> date; npm audit; date;

Monday, March 14, 2022 9:50:52 AM
npm ERR! code ENOLOCK
npm ERR! audit This command requires an existing lockfile.
npm ERR! audit Try creating one first with: npm i --package-lock-only
npm ERR! audit Original error: loadVirtual requires existing shrinkwrap file

npm ERR! A complete log of this run can be found in:
npm ERR!     C:\Users\kushal\scoop\persist\nodejs-lts\cache\_logs\2022-03-14T14_50_53_532Z-debug-0.log
Monday, March 14, 2022 9:50:53 AM

PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date; yarn run ng test; date;                             

Monday, March 14, 2022 9:52:40 AM
yarn run v1.22.17
$ ng update
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json and everything seems to be in order. Good work!
Done in 12.02s.
Monday, March 14, 2022 9:52:52 AM
yarn run v1.22.17
$ ng test
✔ Browser application bundle generation complete.

./src/styles.sass.webpack[javascript/auto]!=!./node_modules/css-loader/dist/cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!./node_modules/postcss-loader/dist/cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!./node_modules/resolve-url-loader/index.js??ruleSet[1].rules[7].rules[1].use[0]!./node_modules/sass-loader/dist/cjs.js??ruleSet[1].rules[7].rules[1].use[1]!./src/styles.sass - Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39

./src/styles.sass - Error: Module build failed (from ./node_modules/mini-css-extract-plugin/dist/loader.js):
HookWebpackError: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39
    at tryRunOrWebpackError (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\HookWebpackError.js:88:9)
    at __webpack_require_module__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5023:12)
    at __webpack_require__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4980:18)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5051:20
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3485:9)
    at done (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3527:9)
    at Hook.eval [as callAsync] (eval at create (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\HookCodeFactory.js:33:10), <anonymous>:15:1)
    at Hook.CALL_ASYNC_DELEGATE [as _callAsync] (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\Hook.js:18:14)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4958:43
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3482:9)
-- inner error --
Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39
    at Object.<anonymous> (C:\Users\kushal\src\beginningmachinelearning\node_modules\css-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!C:\Users\kushal\src\beginningmachinelearning\node_modules\resolve-url-loader\index.js??ruleSet[1].rules[7].rules[1].use[0]!C:\Users\kushal\src\beginningmachinelearning\node_modules\sass-loader\dist\cjs.js??ruleSet[1].rules[7].rules[1].use[1]!C:\Users\kushal\src\beginningmachinelearning\src\styles.sass:1:7)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\javascript\JavascriptModulesPlugin.js:432:11
    at Hook.eval [as call] (eval at create (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\HookCodeFactory.js:19:10), <anonymous>:7:1)
    at Hook.CALL_DELEGATE [as _call] (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\Hook.js:14:14)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5025:39
    at tryRunOrWebpackError (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\HookWebpackError.js:83:7)
    at __webpack_require_module__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5023:12)
    at __webpack_require__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4980:18)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5051:20
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3485:9)

Generated code for C:\Users\kushal\src\beginningmachinelearning\node_modules\css-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!C:\Users\kushal\src\beginningmachinelearning\node_modules\resolve-url-loader\index.js??ruleSet[1].rules[7].rules[1].use[0]!C:\Users\kushal\src\beginningmachinelearning\node_modules\sass-loader\dist\cjs.js??ruleSet[1].rules[7].rules[1].use[1]!C:\Users\kushal\src\beginningmachinelearning\src\styles.sass
1 | throw new Error("Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):\nBrowserslistError: Unknown version 81 of android\n    at Function.select (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:1148:17)\n    at C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:341:33\n    at Array.reduce (<anonymous>)\n    at resolve (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:320:18)\n    at browserslist (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:451:21)\n    at Browsers.parse (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\browsers.js:54:12)\n    at new Browsers (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\browsers.js:42:26)\n    at loadPrefixes (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\autoprefixer.js:106:20)\n    at Object.prepare (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\autoprefixer.js:120:22)\n    at C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\postcss\\lib\\lazy-result.js:149:39");

./src/app/app.component.sass?ngResource - Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39


14 03 2022 09:53:25.029:WARN [karma]: No captured browser, open http://localhost:9876/
14 03 2022 09:53:25.201:INFO [karma-server]: Karma v6.3.2 server started at http://localhost:9876/
14 03 2022 09:53:25.202:INFO [launcher]: Launching browsers FirefoxHeadless with concurrency unlimited
14 03 2022 09:53:25.225:INFO [launcher]: Starting browser Firefox
14 03 2022 09:53:25.233:ERROR [launcher]: Cannot start Firefox
        Can not find the binary C:\Program Files\Mozilla Firefox\firefox.exe
        Please set env variable FIREFOX_BIN
14 03 2022 09:53:25.234:ERROR [launcher]: Firefox stdout:
14 03 2022 09:53:25.235:ERROR [launcher]: Firefox stderr:
PS C:\Users\kushal\src\beginningmachinelearning> 
PS C:\Users\kushal\src\beginningmachinelearning> date; yarn run ng update; date; yarn run ng test; date;

Monday, March 14, 2022 9:58:20 AM
yarn run v1.22.17
$ ng update
Using package manager: 'yarn'
Collecting installed dependencies...
Found 33 dependencies.
    We analyzed your package.json and everything seems to be in order. Good work!
Done in 5.09s.
Monday, March 14, 2022 9:58:25 AM
yarn run v1.22.17
$ ng test
✔ Browser application bundle generation complete.

./src/styles.sass.webpack[javascript/auto]!=!./node_modules/css-loader/dist/cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!./node_modules/postcss-loader/dist/cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!./node_modules/resolve-url-loader/index.js??ruleSet[1].rules[7].rules[1].use[0]!./node_modules/sass-loader/dist/cjs.js??ruleSet[1].rules[7].rules[1].use[1]!./src/styles.sass - Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39

./src/styles.sass - Error: Module build failed (from ./node_modules/mini-css-extract-plugin/dist/loader.js):
HookWebpackError: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39
    at tryRunOrWebpackError (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\HookWebpackError.js:88:9)
    at __webpack_require_module__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5023:12)
    at __webpack_require__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4980:18)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5051:20
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3485:9)
    at done (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3527:9)
    at Hook.eval [as callAsync] (eval at create (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\HookCodeFactory.js:33:10), <anonymous>:15:1)
    at Hook.CALL_ASYNC_DELEGATE [as _callAsync] (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\Hook.js:18:14)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4958:43
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3482:9)
-- inner error --
Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39
    at Object.<anonymous> (C:\Users\kushal\src\beginningmachinelearning\node_modules\css-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!C:\Users\kushal\src\beginningmachinelearning\node_modules\resolve-url-loader\index.js??ruleSet[1].rules[7].rules[1].use[0]!C:\Users\kushal\src\beginningmachinelearning\node_modules\sass-loader\dist\cjs.js??ruleSet[1].rules[7].rules[1].use[1]!C:\Users\kushal\src\beginningmachinelearning\src\styles.sass:1:7)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\javascript\JavascriptModulesPlugin.js:432:11
    at Hook.eval [as call] (eval at create (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\HookCodeFactory.js:19:10), <anonymous>:7:1)
    at Hook.CALL_DELEGATE [as _call] (C:\Users\kushal\src\beginningmachinelearning\node_modules\tapable\lib\Hook.js:14:14)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5025:39
    at tryRunOrWebpackError (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\HookWebpackError.js:83:7)
    at __webpack_require_module__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5023:12)
    at __webpack_require__ (C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:4980:18)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\webpack\lib\Compilation.js:5051:20
    at symbolIterator (C:\Users\kushal\src\beginningmachinelearning\node_modules\neo-async\async.js:3485:9)

Generated code for C:\Users\kushal\src\beginningmachinelearning\node_modules\css-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[1]!C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss-loader\dist\cjs.js??ruleSet[1].rules[7].rules[0].oneOf[0].use[2]!C:\Users\kushal\src\beginningmachinelearning\node_modules\resolve-url-loader\index.js??ruleSet[1].rules[7].rules[1].use[0]!C:\Users\kushal\src\beginningmachinelearning\node_modules\sass-loader\dist\cjs.js??ruleSet[1].rules[7].rules[1].use[1]!C:\Users\kushal\src\beginningmachinelearning\src\styles.sass
1 | throw new Error("Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):\nBrowserslistError: Unknown version 81 of android\n    at Function.select (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:1148:17)\n    at C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:341:33\n    at Array.reduce (<anonymous>)\n    at resolve (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:320:18)\n    at browserslist (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\browserslist\\index.js:451:21)\n    at Browsers.parse (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\browsers.js:54:12)\n    at new Browsers (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\browsers.js:42:26)\n    at loadPrefixes (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\autoprefixer.js:106:20)\n    at Object.prepare (C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\autoprefixer\\lib\\autoprefixer.js:120:22)\n    at C:\\Users\\kushal\\src\\beginningmachinelearning\\node_modules\\postcss\\lib\\lazy-result.js:149:39");

./src/app/app.component.sass?ngResource - Error: Module build failed (from ./node_modules/postcss-loader/dist/cjs.js):
BrowserslistError: Unknown version 81 of android
    at Function.select (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:1148:17)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:341:33
    at Array.reduce (<anonymous>)
    at resolve (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:320:18)
    at browserslist (C:\Users\kushal\src\beginningmachinelearning\node_modules\browserslist\index.js:451:21)
    at Browsers.parse (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:54:12)
    at new Browsers (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\browsers.js:42:26)
    at loadPrefixes (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:106:20)
    at Object.prepare (C:\Users\kushal\src\beginningmachinelearning\node_modules\autoprefixer\lib\autoprefixer.js:120:22)
    at C:\Users\kushal\src\beginningmachinelearning\node_modules\postcss\lib\lazy-result.js:149:39


14 03 2022 09:58:32.967:WARN [karma]: No captured browser, open http://localhost:9876/
14 03 2022 09:58:33.002:INFO [karma-server]: Karma v6.3.2 server started at http://localhost:9876/
14 03 2022 09:58:33.003:INFO [launcher]: Launching browsers FirefoxHeadless with concurrency unlimited
14 03 2022 09:58:33.008:INFO [launcher]: Starting browser Firefox
14 03 2022 09:58:33.015:ERROR [launcher]: Cannot start Firefox
        Can not find the binary C:\Program Files\Mozilla Firefox\firefox.exe
        Please set env variable FIREFOX_BIN
14 03 2022 09:58:33.016:ERROR [launcher]: Firefox stdout:
14 03 2022 09:58:33.016:ERROR [launcher]: Firefox stderr:
PS C:\Users\kushal\src\beginningmachinelearning> 