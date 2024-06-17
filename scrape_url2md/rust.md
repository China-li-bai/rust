快速入门 – React 中文文档window.dataLayer = window.dataLayer || [];function gtag(){dataLayer.push(arguments);}gtag('js', new Date());gtag('config', 'G-B1E83PJ3RT'); (function () { try { let logShown = false; function setUwu(isUwu) { try { if (isUwu) { localStorage.setItem('uwu', true); document.documentElement.classList.add('uwu'); if (!logShown) { console.log('uwu mode! turn off with ?uwu=0'); console.log('logo credit to @sawaratsuki1004 via https://github.com/SAWARATSUKI/ServiceLogos'); logShown = true; } } else { localStorage.removeItem('uwu'); document.documentElement.classList.remove('uwu'); console.log('uwu mode off. turn on with ?uwu'); } } catch (err) { } } window.\_\_setUwu = setUwu; function checkQueryParam() { const params = new URLSearchParams(window.location.search); const value = params.get('uwu'); switch(value) { case '': case 'true': case '1': return true; case 'false': case '0': return false; default: return null; } } function checkLocalStorage() { try { return localStorage.getItem('uwu') === 'true'; } catch (err) { return false; } } const uwuQueryParam = checkQueryParam(); console.log('uwuQueryParam', uwuQueryParam); if (uwuQueryParam != null) { setUwu(uwuQueryParam); } else if (checkLocalStorage()) { document.documentElement.classList.add('uwu'); } } catch (err) { } })();  (function () { function setTheme(newTheme) { window.\_\_theme = newTheme; if (newTheme === 'dark') { document.documentElement.classList.add('dark'); } else if (newTheme === 'light') { document.documentElement.classList.remove('dark'); } } var preferredTheme; try { preferredTheme = localStorage.getItem('theme'); } catch (err) { } window.\_\_setPreferredTheme = function(newTheme) { preferredTheme = newTheme; setTheme(newTheme); try { localStorage.setItem('theme', newTheme); } catch (err) { } }; var initialTheme = preferredTheme; var darkQuery = window.matchMedia('(prefers-color-scheme: dark)'); if (!initialTheme) { initialTheme = darkQuery.matches ? 'dark' : 'light'; } setTheme(initialTheme); darkQuery.addEventListener('change', function (e) { if (!preferredTheme) { setTheme(e.matches ? 'dark' : 'light'); } }); // Detect whether the browser is Mac to display platform specific content // An example of such content can be the keyboard shortcut displayed in the search bar document.documentElement.classList.add( window.navigator.platform.includes('Mac') ? "platform-mac" : "platform-win" ); })();

[<img alt="logo by @sawaratsuki1004" src="/_next/image?url=%2Fimages%2Fuwu.png&w=128&q=75" title="logo by @sawaratsuki1004" height="32" width="63" />](/)

[React](/)

[v18.3.1](/versions)

搜索⌘CtrlK

[教程](/learn)

[参考](/reference/react)

[社区](/community)

[博客](/blog)

[](/community/translations)

[](https://github.com/facebook/react/releases)

### 起步 ###

* [

  快速入门

  ](/learn)

  * [

    教程：井字棋游戏

    ](/learn/tutorial-tic-tac-toe)
  * [

    React 哲学

    ](/learn/thinking-in-react)

* [

  安装

  ](/learn/installation)

  * [

    启动一个新的 React 项目

    ](/learn/start-a-new-react-project)
  * [

    将 React 添加到现有项目中

    ](/learn/add-react-to-an-existing-project)
  * [

    编辑器设置

    ](/learn/editor-setup)
  * [

    使用 TypeScript

    ](/learn/typescript)
  * [

    React 开发者工具

    ](/learn/react-developer-tools)
  * [

    React Compiler  - This feature is available in the latest Canary

    ](/learn/react-compiler)

*

### 学习 React ###

* [

  描述 UI

  ](/learn/describing-the-ui)

  * [

    你的第一个组件

    ](/learn/your-first-component)
  * [

    组件的导入与导出

    ](/learn/importing-and-exporting-components)
  * [

    使用 JSX 书写标签语言

    ](/learn/writing-markup-with-jsx)
  * [

    在 JSX 中通过大括号使用 JavaScript

    ](/learn/javascript-in-jsx-with-curly-braces)
  * [

    将 Props 传递给组件

    ](/learn/passing-props-to-a-component)
  * [

    条件渲染

    ](/learn/conditional-rendering)
  * [

    渲染列表

    ](/learn/rendering-lists)
  * [

    保持组件纯粹

    ](/learn/keeping-components-pure)
  * [

    将 UI 视为树

    ](/learn/understanding-your-ui-as-a-tree)

* [

  添加交互

  ](/learn/adding-interactivity)

  * [

    响应事件

    ](/learn/responding-to-events)
  * [

    state：组件的记忆

    ](/learn/state-a-components-memory)
  * [

    渲染和提交

    ](/learn/render-and-commit)
  * [

    state 如同一张快照

    ](/learn/state-as-a-snapshot)
  * [

    把一系列 state 更新加入队列

    ](/learn/queueing-a-series-of-state-updates)
  * [

    更新 state 中的对象

    ](/learn/updating-objects-in-state)
  * [

    更新 state 中的数组

    ](/learn/updating-arrays-in-state)

* [

  状态管理

  ](/learn/managing-state)

  * [

    用 State 响应输入

    ](/learn/reacting-to-input-with-state)
  * [

    选择 State 结构

    ](/learn/choosing-the-state-structure)
  * [

    在组件间共享状态

    ](/learn/sharing-state-between-components)
  * [

    对 state 进行保留和重置

    ](/learn/preserving-and-resetting-state)
  * [

    迁移状态逻辑至 Reducer 中

    ](/learn/extracting-state-logic-into-a-reducer)
  * [

    使用 Context 深层传递参数

    ](/learn/passing-data-deeply-with-context)
  * [

    使用 Reducer 和 Context 拓展你的应用

    ](/learn/scaling-up-with-reducer-and-context)

* [

  脱围机制

  ](/learn/escape-hatches)

  * [

    使用 ref 引用值

    ](/learn/referencing-values-with-refs)
  * [

    使用 ref 操作 DOM

    ](/learn/manipulating-the-dom-with-refs)
  * [

    使用 Effect 同步

    ](/learn/synchronizing-with-effects)
  * [

    你可能不需要 Effect

    ](/learn/you-might-not-need-an-effect)
  * [

    响应式 Effect 的生命周期

    ](/learn/lifecycle-of-reactive-effects)
  * [

    将事件从 Effect 中分开

    ](/learn/separating-events-from-effects)
  * [

    移除 Effect 依赖

    ](/learn/removing-effect-dependencies)
  * [

    使用自定义 Hook 复用逻辑

    ](/learn/reusing-logic-with-custom-hooks)

这个页面对你有帮助吗？

[学习 React](/learn)

快速入门[](#undefined)
==========

欢迎来到 React 文档！本章节将介绍你每天都会使用的 80% 的 React 概念。

### 你将会学习到 ###

* 如何创建和嵌套组件
* 如何添加标签和样式
* 如何显示数据
* 如何渲染条件和列表
* 如何对事件做出响应并更新界面
* 如何在组件间共享数据

创建和嵌套组件 [](#components)
----------

React 应用程序是由 **组件** 组成的。一个组件是 UI（用户界面）的一部分，它拥有自己的逻辑和外观。组件可以小到一个按钮，也可以大到整个页面。

React 组件是返回标签的 JavaScript 函数：

```
function MyButton() {  return (    <button>I'm a button</button>  );}
```

至此，你已经声明了 `MyButton`，现在把它嵌套到另一个组件中：

```
export default function MyApp() {  return (    <div>      <h1>Welcome to my app</h1>      <MyButton />    </div>  );}
```

你可能已经注意到 `<MyButton />` 是以大写字母开头的。你可以据此识别 React 组件。React 组件必须以大写字母开头，而 HTML 标签则必须是小写字母。

来看下效果：

App.js

App.js

 重置[Fork](https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app)

```
function MyButton() {
  return (
    <button>
      I'm a button
    </button>
  );
}

export default function MyApp() {
  return (
    <div>
      <h1>Welcome to my app</h1>
      <MyButton />
    </div>
  );
}

```

显示更多

`export default` 关键字指定了文件中的主要组件。如果你对 JavaScript 某些语法不熟悉，可以参考 [MDN](https://developer.mozilla.org/zh-CN/docs/web/javascript/reference/statements/export) 和 [javascript.info](https://javascript.info/import-export)。

使用 JSX 编写标签 [](#writing-markup-with-jsx)
----------

上面所使用的标签语法被称为 *JSX*。它是可选的，但大多数 React 项目会使用 JSX，主要是它很方便。所有 [我们推荐的本地开发工具](/learn/installation) 都开箱即用地支持 JSX。

JSX 比 HTML 更加严格。你必须闭合标签，如 `<br />`。你的组件也不能返回多个 JSX 标签。你必须将它们包裹到一个共享的父级中，比如 `<div>...</div>` 或使用空的 `<>...</>` 包裹：

```
function AboutPage() {  return (    <>      <h1>About</h1>      <p>Hello there.<br />How do you do?</p>    </>  );}
```

如果你有大量的 HTML 需要移植到 JSX 中，你可以使用 [在线转换器](https://transform.tools/html-to-jsx)。

添加样式 [](#adding-styles)
----------

在 React 中，你可以使用 `className` 来指定一个 CSS 的 class。它与 HTML 的 [`class`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Global_attributes/class) 属性的工作方式相同：

```
<img className="avatar" />
```

然后，你可以在一个单独的 CSS 文件中为它编写 CSS 规则：

```
/* In your CSS */.avatar {  border-radius: 50%;}
```

React 并没有规定你如何添加 CSS 文件。最简单的方式是使用 HTML 的 [`<link>`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/link) 标签。如果你使用了构建工具或框架，请阅读其文档来了解如何将 CSS 文件添加到你的项目中。

显示数据 [](#displaying-data)
----------

JSX 会让你把标签放到 JavaScript 中。而大括号会让你 “回到” JavaScript 中，这样你就可以从你的代码中嵌入一些变量并展示给用户。例如，这将显示 `user.name`：

```
return (  <h1>    {user.name}  </h1>);
```

你还可以将 JSX 属性 “转义到 JavaScript”，但你必须使用大括号 **而非** 引号。例如，`className="avatar"` 是将 `"avatar"` 字符串传递给 `className`，作为 CSS 的 class。但 `src={user.imageUrl}` 会读取 JavaScript 的 `user.imageUrl` 变量，然后将该值作为 `src` 属性传递：

```
return (  <img    className="avatar"    src={user.imageUrl}  />);
```

你也可以把更为复杂的表达式放入 JSX 的大括号内，例如 [字符串拼接](https://javascript.info/operators#string-concatenation-with-binary)：

App.js

App.js

 重置[Fork](https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app)

```
const user = {
  name: 'Hedy Lamarr',
  imageUrl: 'https://i.imgur.com/yXOvdOSs.jpg',
  imageSize: 90,
};

export default function Profile() {
  return (
    <>
      <h1>{user.name}</h1>
      <img
        className="avatar"
        src={user.imageUrl}
        alt={'Photo of ' + user.name}
        style={{
          width: user.imageSize,
          height: user.imageSize
        }}
      />
    </>
  );
}

```

显示更多

在上面示例中，`style={{}}` 并不是一个特殊的语法，而是 `style={ }` JSX 大括号内的一个普通 `{}` 对象。当你的样式依赖于 JavaScript 变量时，你可以使用 `style` 属性。

条件渲染 [](#conditional-rendering)
----------

React 没有特殊的语法来编写条件语句，因此你使用的就是普通的 JavaScript 代码。例如使用 [`if`](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/if...else) 语句根据条件引入 JSX：

```
let content;if (isLoggedIn) {  content = <AdminPanel />;} else {  content = <LoginForm />;}return (  <div>    {content}  </div>);
```

如果你喜欢更为紧凑的代码，可以使用 [条件 `?` 运算符](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Conditional_Operator)。与 `if` 不同的是，它工作于 JSX 内部：

```
<div>  {isLoggedIn ? (    <AdminPanel />  ) : (    <LoginForm />  )}</div>
```

当你不需要 `else` 分支时，你还可以使用 [逻辑 `&&` 语法](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Logical_AND#short-circuit_evaluation)：

```
<div>  {isLoggedIn && <AdminPanel />}</div>
```

所有这些方法也适用于有条件地指定属性。如果你对 JavaScript 语法不熟悉，你可以从一直使用 `if...else` 开始。

渲染列表 [](#rendering-lists)
----------

你将依赖 JavaScript 的特性，例如 [`for` 循环](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/for) 和 [array 的 `map()` 函数](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Array/map) 来渲染组件列表。

假设你有一个产品数组：

```
const products = [  { title: 'Cabbage', id: 1 },  { title: 'Garlic', id: 2 },  { title: 'Apple', id: 3 },];
```

在你的组件中，使用 `map()` 函数将这个数组转换为 `<li>` 标签构成的列表:

```
const listItems = products.map(product =>  <li key={product.id}>    {product.title}  </li>);return (  <ul>{listItems}</ul>);
```

注意， `<li>` 有一个 `key` 属性。对于列表中的每一个元素，你都应该传递一个字符串或者数字给 `key`，用于在其兄弟节点中唯一标识该元素。通常 key 来自你的数据，比如数据库中的 ID。如果你在后续插入、删除或重新排序这些项目，React 将依靠你提供的 key 来思考发生了什么。

App.js

App.js

 重置[Fork](https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app)

```
const products = [
  { title: 'Cabbage', isFruit: false, id: 1 },
  { title: 'Garlic', isFruit: false, id: 2 },
  { title: 'Apple', isFruit: true, id: 3 },
];

export default function ShoppingList() {
  const listItems = products.map(product =>
    <li
      key={product.id}
      style={{
        color: product.isFruit ? 'magenta' : 'darkgreen'
      }}
    >
      {product.title}
    </li>
  );

  return (
    <ul>{listItems}</ul>
  );
}

```

显示更多

响应事件 [](#responding-to-events)
----------

你可以通过在组件中声明 **事件处理** 函数来响应事件：

```
function MyButton() {  function handleClick() {    alert('You clicked me!');  }  return (    <button onClick={handleClick}>      Click me    </button>  );}
```

注意，`onClick={handleClick}` 的结尾没有小括号！不要 **调用** 事件处理函数：你只需 **把函数传递给事件** 即可。当用户点击按钮时 React 会调用你传递的事件处理函数。

更新界面 [](#updating-the-screen)
----------

通常你会希望你的组件 “记住” 一些信息并展示出来，比如一个按钮被点击的次数。要做到这一点，你需要在你的组件中添加 **state**。

首先，从 React 引入 [`useState`](/reference/react/useState)：

```
import { useState } from 'react';
```

现在你可以在你的组件中声明一个 **state 变量**：

```
function MyButton() {  const [count, setCount] = useState(0);  // ...
```

你将从 `useState` 中获得两样东西：当前的 state（`count`），以及用于更新它的函数（`setCount`）。你可以给它们起任何名字，但按照惯例会像 `[something, setSomething]` 这样为它们命名。

第一次显示按钮时，`count` 的值为 `0`，因为你把 `0` 传给了 `useState()`。当你想改变 state 时，调用 `setCount()` 并将新的值传递给它。点击该按钮计数器将递增：

```
function MyButton() {  const [count, setCount] = useState(0);  function handleClick() {    setCount(count + 1);  }  return (    <button onClick={handleClick}>      Clicked {count} times    </button>  );}
```

React 将再次调用你的组件函数。第一次 `count` 变成 `1`。接着点击会变成 `2`。继续点击会逐步递增。

如果你多次渲染同一个组件，每个组件都会拥有自己的 state。你可以尝试点击不同的按钮：

App.js

App.js

 重置[Fork](https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app)

```
import { useState } from 'react';

export default function MyApp() {
  return (
    <div>
      <h1>Counters that update separately</h1>
      <MyButton />
      <MyButton />
    </div>
  );
}

function MyButton() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }

  return (
    <button onClick={handleClick}>
      Clicked {count} times
    </button>
  );
}

```

显示更多

注意，每个按钮会 “记住” 自己的 `count`，而不影响其他按钮。

使用 Hook [](#using-hooks)
----------

以 `use` 开头的函数被称为 **Hook**。`useState` 是 React 提供的一个内置 Hook。你可以在 [React API 参考](/reference/react) 中找到其他内置的 Hook。你也可以通过组合现有的 Hook 来编写属于你自己的 Hook。

Hook 比普通函数更为严格。你只能在你的组件（或其他 Hook）的 **顶层** 调用 Hook。如果你想在一个条件或循环中使用 `useState`，请提取一个新的组件并在组件内部使用它。

组件间共享数据 [](#sharing-data-between-components)
----------

在前面的示例中，每个 `MyButton` 都有自己独立的 `count`，当每个按钮被点击时，只有被点击按钮的 `count` 才会发生改变：

<img alt="Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. Both MyButton components contain a count with value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child.dark.png&w=828&q=75" height="367" width="407" />

<img alt="Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. Both MyButton components contain a count with value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child.png&w=828&q=75" height="367" width="407" />

起初，每个 `MyButton` 的 `count` state 均为 `0`

<img alt="The same diagram as the previous, with the count of the first child MyButton component highlighted indicating a click with the count value incremented to one. The second MyButton component still contains value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child_clicked.dark.png&w=828&q=75" height="367" width="407" />

<img alt="The same diagram as the previous, with the count of the first child MyButton component highlighted indicating a click with the count value incremented to one. The second MyButton component still contains value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child_clicked.png&w=828&q=75" height="367" width="407" />

第一个 `MyButton` 会将 `count` 更新为 `1`

然而，你经常需要组件 **共享数据并一起更新**。

为了使得 `MyButton` 组件显示相同的 `count` 并一起更新，你需要将各个按钮的 state “向上” 移动到最接近包含所有按钮的组件之中。

在这个示例中，它是 `MyApp`：

<img alt="Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. MyApp contains a count value of zero which is passed down to both of the MyButton components, which also show value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent.dark.png&w=828&q=75" height="385" width="410" />

<img alt="Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. MyApp contains a count value of zero which is passed down to both of the MyButton components, which also show value zero." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent.png&w=828&q=75" height="385" width="410" />

起初，`MyApp` 的 `count` state 为 `0` 并传递给了两个子组件

<img alt="The same diagram as the previous, with the count of the parent MyApp component highlighted indicating a click with the value incremented to one. The flow to both of the children MyButton components is also highlighted, and the count value in each child is set to one indicating the value was passed down." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent_clicked.dark.png&w=828&q=75" height="385" width="410" />

<img alt="The same diagram as the previous, with the count of the parent MyApp component highlighted indicating a click with the value incremented to one. The flow to both of the children MyButton components is also highlighted, and the count value in each child is set to one indicating the value was passed down." src="/_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent_clicked.png&w=828&q=75" height="385" width="410" />

点击后，`MyApp` 将 `count` state 更新为 `1`，并将其传递给两个子组件

此刻，当你点击任何一个按钮时，`MyApp` 中的 `count` 都将改变，同时会改变 `MyButton` 中的两个 count。具体代码如下：

首先，将 `MyButton` 的 **state 上移到** `MyApp` 中：

```
export default function MyApp() {  const [count, setCount] = useState(0);  function handleClick() {    setCount(count + 1);  }  return (    <div>      <h1>Counters that update separately</h1>      <MyButton />      <MyButton />    </div>  );}function MyButton() {  // ... we're moving code from here ...}
```

接着，将 `MyApp` 中的点击事件处理函数以及 **state 一同向下传递到** 每个 `MyButton` 中。你可以使用 JSX 的大括号向 `MyButton` 传递信息。就像之前向 `<img>` 等内置标签所做的那样:

```
export default function MyApp() {  const [count, setCount] = useState(0);  function handleClick() {    setCount(count + 1);  }  return (    <div>      <h1>Counters that update together</h1>      <MyButton count={count} onClick={handleClick} />      <MyButton count={count} onClick={handleClick} />    </div>  );}
```

使用这种方式传递的信息被称作 **prop**。此时 `MyApp` 组件包含了 `count` state 以及 `handleClick` 事件处理函数，并将它们作为 **prop 传递给** 了每个按钮。

最后，改变 `MyButton` 以 **读取** 从父组件传递来的 prop：

```
function MyButton({ count, onClick }) {  return (    <button onClick={onClick}>      Clicked {count} times    </button>  );}
```

当你点击按钮时，`onClick` 处理程序会启动。每个按钮的 `onClick` prop 会被设置为 `MyApp` 内的 `handleClick` 函数，所以函数内的代码会被执行。该代码会调用 `setCount(count + 1)`，使得 state 变量 `count` 递增。新的 `count` 值会被作为 prop 传递给每个按钮，因此它们每次展示的都是最新的值。这被称为“状态提升”。通过向上移动 state，我们实现了在组件间共享它。

App.js

App.js

 重置[Fork](https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app)

```
import { useState } from 'react';

export default function MyApp() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }

  return (
    <div>
      <h1>Counters that update together</h1>
      <MyButton count={count} onClick={handleClick} />
      <MyButton count={count} onClick={handleClick} />
    </div>
  );
}

function MyButton({ count, onClick }) {
  return (
    <button onClick={onClick}>
      Clicked {count} times
    </button>
  );
}

```

显示更多

下一节 [](#next-steps)
----------

至此，你已经了解了如何编写 React 代码的基本知识。

接下来你可以查看 [实战教程](/learn/tutorial-tic-tac-toe) 并把它们付诸实践，用 React 建立第一个迷你应用程序。

[

下一页教程：井字棋游戏

](/learn/tutorial-tic-tac-toe)

---

[

](https://opensource.fb.com/)

©2024

no uwu plz

uwu?

Logo by[@sawaratsuki1004](https://twitter.com/sawaratsuki1004)

[学习 React](/learn)

[快速入门](/learn)

[安装](/learn/installation)

[描述 UI](/learn/describing-the-ui)

[添加交互](/learn/adding-interactivity)

[状态管理](/learn/managing-state)

[脱围机制](/learn/escape-hatches)

[API 参考](/reference/react)

[React API](/reference/react)

[React DOM API](/reference/react-dom)

[社区](/community)

[行为准则](https://github.com/facebook/react/blob/main/CODE_OF_CONDUCT.md)

[团队](/community/team)

[文档贡献者](/community/docs-contributors)

[鸣谢](/community/acknowledgements)

了解更多

[博客](/blog)

[React Native](https://reactnative.dev/)

[隐私政策](https://opensource.facebook.com/legal/privacy)

[条款](https://opensource.fb.com/legal/terms/)

[](https://www.facebook.com/react)[](https://twitter.com/reactjs)[](https://github.com/facebook/react)

目录
----------

* [概览](#)
* [创建和嵌套组件 ](#components)
* [使用 JSX 编写标签 ](#writing-markup-with-jsx)
* [添加样式 ](#adding-styles)
* [显示数据 ](#displaying-data)
* [条件渲染 ](#conditional-rendering)
* [渲染列表 ](#rendering-lists)
* [响应事件 ](#responding-to-events)
* [更新界面 ](#updating-the-screen)
* [使用 Hook ](#using-hooks)
* [组件间共享数据 ](#sharing-data-between-components)
* [下一节 ](#next-steps)

{"props":{"pageProps":{"toc":"[{\\"url\\":\\"#\\",\\"text\\":\\"概览\\",\\"depth\\":2},{\\"url\\":\\"#components\\",\\"depth\\":2,\\"text\\":\\"创建和嵌套组件 \\"},{\\"url\\":\\"#writing-markup-with-jsx\\",\\"depth\\":2,\\"text\\":\\"使用 JSX 编写标签 \\"},{\\"url\\":\\"#adding-styles\\",\\"depth\\":2,\\"text\\":\\"添加样式 \\"},{\\"url\\":\\"#displaying-data\\",\\"depth\\":2,\\"text\\":\\"显示数据 \\"},{\\"url\\":\\"#conditional-rendering\\",\\"depth\\":2,\\"text\\":\\"条件渲染 \\"},{\\"url\\":\\"#rendering-lists\\",\\"depth\\":2,\\"text\\":\\"渲染列表 \\"},{\\"url\\":\\"#responding-to-events\\",\\"depth\\":2,\\"text\\":\\"响应事件 \\"},{\\"url\\":\\"#updating-the-screen\\",\\"depth\\":2,\\"text\\":\\"更新界面 \\"},{\\"url\\":\\"#using-hooks\\",\\"depth\\":2,\\"text\\":\\"使用 Hook \\"},{\\"url\\":\\"#sharing-data-between-components\\",\\"depth\\":2,\\"text\\":\\"组件间共享数据 \\"},{\\"url\\":\\"#next-steps\\",\\"depth\\":2,\\"text\\":\\"下一节 \\"}]","content":"[[\\"$r\\",\\"MaxWidth\\",\\"20\\",{\\"children\\":[[\\"$r\\",\\"Intro\\",null,{\\"children\\":[\\"$r\\",\\"p\\",null,{\\"children\\":\\"欢迎来到 React 文档！本章节将介绍你每天都会使用的 80% 的 React 概念。\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"YouWillLearn\\",null,{\\"children\\":[\\"$r\\",\\"ul\\",null,{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何创建和嵌套组件\\"}],\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何添加标签和样式\\"}],\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何显示数据\\"}],\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何渲染条件和列表\\"}],\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何对事件做出响应并更新界面\\"}],\\"\\\\n\\",[\\"$r\\",\\"li\\",null,{\\"children\\":\\"如何在组件间共享数据\\"}],\\"\\\\n\\"]}]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"components\\",\\"children\\":\\"创建和嵌套组件 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"React 应用程序是由 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"组件\\"}],\\" 组成的。一个组件是 UI（用户界面）的一部分，它拥有自己的逻辑和外观。组件可以小到一个按钮，也可以大到整个页面。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"React 组件是返回标签的 JavaScript 函数：\\"}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"function MyButton() {\\\\n return (\\\\n \\u003cbutton\\u003eI'm a button\\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"至此，你已经声明了 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\"，现在把它嵌套到另一个组件中：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{5}\\",\\"children\\":\\"export default function MyApp() {\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eWelcome to my app\\u003c/h1\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你可能已经注意到 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cMyButton /\\u003e\\"}],\\" 是以大写字母开头的。你可以据此识别 React 组件。React 组件必须以大写字母开头，而 HTML 标签则必须是小写字母。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"来看下效果：\\"}],\\"\\\\n\\"]}],[\\"$r\\",\\"Sandpack\\",null,{\\"children\\":[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"function MyButton() {\\\\n return (\\\\n \\u003cbutton\\u003e\\\\n I'm a button\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\\\nexport default function MyApp() {\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eWelcome to my app\\u003c/h1\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\"}]}]}],[\\"$r\\",\\"MaxWidth\\",\\"58\\",{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[[\\"$r\\",\\"code\\",null,{\\"children\\":\\"export default\\"}],\\" 关键字指定了文件中的主要组件。如果你对 JavaScript 某些语法不熟悉，可以参考 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/web/javascript/reference/statements/export\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":\\"MDN\\"}],\\" 和 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://javascript.info/import-export\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":\\"javascript.info\\"}],\\"。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"writing-markup-with-jsx\\",\\"children\\":\\"使用 JSX 编写标签 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"上面所使用的标签语法被称为 \\",[\\"$r\\",\\"em\\",null,{\\"children\\":\\"JSX\\"}],\\"。它是可选的，但大多数 React 项目会使用 JSX，主要是它很方便。所有 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"/learn/installation\\",\\"children\\":\\"我们推荐的本地开发工具\\"}],\\" 都开箱即用地支持 JSX。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"JSX 比 HTML 更加严格。你必须闭合标签，如 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cbr /\\u003e\\"}],\\"。你的组件也不能返回多个 JSX 标签。你必须将它们包裹到一个共享的父级中，比如 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cdiv\\u003e...\\u003c/div\\u003e\\"}],\\" 或使用空的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003c\\u003e...\\u003c/\\u003e\\"}],\\" 包裹：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{3,6}\\",\\"children\\":\\"function AboutPage() {\\\\n return (\\\\n \\u003c\\u003e\\\\n \\u003ch1\\u003eAbout\\u003c/h1\\u003e\\\\n \\u003cp\\u003eHello there.\\u003cbr /\\u003eHow do you do?\\u003c/p\\u003e\\\\n \\u003c/\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"如果你有大量的 HTML 需要移植到 JSX 中，你可以使用 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://transform.tools/html-to-jsx\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":\\"在线转换器\\"}],\\"。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"adding-styles\\",\\"children\\":\\"添加样式 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"在 React 中，你可以使用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"className\\"}],\\" 来指定一个 CSS 的 class。它与 HTML 的 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/HTML/Global\_attributes/class\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"$r\\",\\"code\\",null,{\\"children\\":\\"class\\"}]}],\\" 属性的工作方式相同：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"\\u003cimg className=\\\\\\"avatar\\\\\\" /\\u003e\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"然后，你可以在一个单独的 CSS 文件中为它编写 CSS 规则：\\"}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-css\\",\\"children\\":\\"/\* In your CSS \*/\\\\n.avatar {\\\\n border-radius: 50%;\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"React 并没有规定你如何添加 CSS 文件。最简单的方式是使用 HTML 的 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/link\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003clink\\u003e\\"}]}],\\" 标签。如果你使用了构建工具或框架，请阅读其文档来了解如何将 CSS 文件添加到你的项目中。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"displaying-data\\",\\"children\\":\\"显示数据 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"JSX 会让你把标签放到 JavaScript 中。而大括号会让你 “回到” JavaScript 中，这样你就可以从你的代码中嵌入一些变量并展示给用户。例如，这将显示 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"user.name\\"}],\\"：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{3}\\",\\"children\\":\\"return (\\\\n \\u003ch1\\u003e\\\\n {user.name}\\\\n \\u003c/h1\\u003e\\\\n);\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你还可以将 JSX 属性 “转义到 JavaScript”，但你必须使用大括号 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"而非\\"}],\\" 引号。例如，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"className=\\\\\\"avatar\\\\\\"\\"}],\\" 是将 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\\\\\"avatar\\\\\\"\\"}],\\" 字符串传递给 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"className\\"}],\\"，作为 CSS 的 class。但 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"src={user.imageUrl}\\"}],\\" 会读取 JavaScript 的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"user.imageUrl\\"}],\\" 变量，然后将该值作为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"src\\"}],\\" 属性传递：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{3,4}\\",\\"children\\":\\"return (\\\\n \\u003cimg\\\\n className=\\\\\\"avatar\\\\\\"\\\\n src={user.imageUrl}\\\\n /\\u003e\\\\n);\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你也可以把更为复杂的表达式放入 JSX 的大括号内，例如 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://javascript.info/operators#string-concatenation-with-binary\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":\\"字符串拼接\\"}],\\"：\\"]}],\\"\\\\n\\"]}],[\\"$r\\",\\"Sandpack\\",null,{\\"children\\":[[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"const user = {\\\\n name: 'Hedy Lamarr',\\\\n imageUrl: 'https://i.imgur.com/yXOvdOSs.jpg',\\\\n imageSize: 90,\\\\n};\\\\n\\\\nexport default function Profile() {\\\\n return (\\\\n \\u003c\\u003e\\\\n \\u003ch1\\u003e{user.name}\\u003c/h1\\u003e\\\\n \\u003cimg\\\\n className=\\\\\\"avatar\\\\\\"\\\\n src={user.imageUrl}\\\\n alt={'Photo of ' + user.name}\\\\n style={{\\\\n width: user.imageSize,\\\\n height: user.imageSize\\\\n }}\\\\n /\\u003e\\\\n \\u003c/\\u003e\\\\n );\\\\n}\\\\n\\"}]}],[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-css\\",\\"children\\":\\".avatar {\\\\n border-radius: 50%;\\\\n}\\\\n\\\\n.large {\\\\n border: 4px solid gold;\\\\n}\\\\n\\"}]}]]}],[\\"$r\\",\\"MaxWidth\\",\\"92\\",{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"在上面示例中，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"style={{}}\\"}],\\" 并不是一个特殊的语法，而是 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"style={ }\\"}],\\" JSX 大括号内的一个普通 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"{}\\"}],\\" 对象。当你的样式依赖于 JavaScript 变量时，你可以使用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"style\\"}],\\" 属性。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"conditional-rendering\\",\\"children\\":\\"条件渲染 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"React 没有特殊的语法来编写条件语句，因此你使用的就是普通的 JavaScript 代码。例如使用 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/if...else\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"$r\\",\\"code\\",null,{\\"children\\":\\"if\\"}]}],\\" 语句根据条件引入 JSX：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"let content;\\\\nif (isLoggedIn) {\\\\n content = \\u003cAdminPanel /\\u003e;\\\\n} else {\\\\n content = \\u003cLoginForm /\\u003e;\\\\n}\\\\nreturn (\\\\n \\u003cdiv\\u003e\\\\n {content}\\\\n \\u003c/div\\u003e\\\\n);\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"如果你喜欢更为紧凑的代码，可以使用 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Conditional\_Operator\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"条件 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"?\\"}],\\" 运算符\\"]}],\\"。与 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"if\\"}],\\" 不同的是，它工作于 JSX 内部：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"\\u003cdiv\\u003e\\\\n {isLoggedIn ? (\\\\n \\u003cAdminPanel /\\u003e\\\\n ) : (\\\\n \\u003cLoginForm /\\u003e\\\\n )}\\\\n\\u003c/div\\u003e\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"当你不需要 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"else\\"}],\\" 分支时，你还可以使用 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Logical\_AND#short-circuit\_evaluation\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"逻辑 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u0026\\u0026\\"}],\\" 语法\\"]}],\\"：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"\\u003cdiv\\u003e\\\\n {isLoggedIn \\u0026\\u0026 \\u003cAdminPanel /\\u003e}\\\\n\\u003c/div\\u003e\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"所有这些方法也适用于有条件地指定属性。如果你对 JavaScript 语法不熟悉，你可以从一直使用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"if...else\\"}],\\" 开始。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"rendering-lists\\",\\"children\\":\\"渲染列表 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你将依赖 JavaScript 的特性，例如 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/for\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[[\\"$r\\",\\"code\\",null,{\\"children\\":\\"for\\"}],\\" 循环\\"]}],\\" 和 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global\_Objects/Array/map\\",\\"target\\":\\"\_blank\\",\\"rel\\":\\"nofollow noopener noreferrer\\",\\"children\\":[\\"array 的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"map()\\"}],\\" 函数\\"]}],\\" 来渲染组件列表。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"假设你有一个产品数组：\\"}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"const products = [\\\\n { title: 'Cabbage', id: 1 },\\\\n { title: 'Garlic', id: 2 },\\\\n { title: 'Apple', id: 3 },\\\\n];\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"在你的组件中，使用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"map()\\"}],\\" 函数将这个数组转换为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cli\\u003e\\"}],\\" 标签构成的列表:\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"const listItems = products.map(product =\\u003e\\\\n \\u003cli key={product.id}\\u003e\\\\n {product.title}\\\\n \\u003c/li\\u003e\\\\n);\\\\n\\\\nreturn (\\\\n \\u003cul\\u003e{listItems}\\u003c/ul\\u003e\\\\n);\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"注意， \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cli\\u003e\\"}],\\" 有一个 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"key\\"}],\\" 属性。对于列表中的每一个元素，你都应该传递一个字符串或者数字给 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"key\\"}],\\"，用于在其兄弟节点中唯一标识该元素。通常 key 来自你的数据，比如数据库中的 ID。如果你在后续插入、删除或重新排序这些项目，React 将依靠你提供的 key 来思考发生了什么。\\"]}],\\"\\\\n\\"]}],[\\"$r\\",\\"Sandpack\\",null,{\\"children\\":[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"const products = [\\\\n { title: 'Cabbage', isFruit: false, id: 1 },\\\\n { title: 'Garlic', isFruit: false, id: 2 },\\\\n { title: 'Apple', isFruit: true, id: 3 },\\\\n];\\\\n\\\\nexport default function ShoppingList() {\\\\n const listItems = products.map(product =\\u003e\\\\n \\u003cli\\\\n key={product.id}\\\\n style={{\\\\n color: product.isFruit ? 'magenta' : 'darkgreen'\\\\n }}\\\\n \\u003e\\\\n {product.title}\\\\n \\u003c/li\\u003e\\\\n );\\\\n\\\\n return (\\\\n \\u003cul\\u003e{listItems}\\u003c/ul\\u003e\\\\n );\\\\n}\\\\n\\"}]}]}],[\\"$r\\",\\"MaxWidth\\",\\"124\\",{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"responding-to-events\\",\\"children\\":\\"响应事件 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你可以通过在组件中声明 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"事件处理\\"}],\\" 函数来响应事件：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{2-4,7}\\",\\"children\\":\\"function MyButton() {\\\\n function handleClick() {\\\\n alert('You clicked me!');\\\\n }\\\\n\\\\n return (\\\\n \\u003cbutton onClick={handleClick}\\u003e\\\\n Click me\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"注意，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"onClick={handleClick}\\"}],\\" 的结尾没有小括号！不要 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"调用\\"}],\\" 事件处理函数：你只需 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"把函数传递给事件\\"}],\\" 即可。当用户点击按钮时 React 会调用你传递的事件处理函数。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"updating-the-screen\\",\\"children\\":\\"更新界面 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"通常你会希望你的组件 “记住” 一些信息并展示出来，比如一个按钮被点击的次数。要做到这一点，你需要在你的组件中添加 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"state\\"}],\\"。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"首先，从 React 引入 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"/reference/react/useState\\",\\"children\\":[\\"$r\\",\\"code\\",null,{\\"children\\":\\"useState\\"}]}],\\"：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"import { useState } from 'react';\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"现在你可以在你的组件中声明一个 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"state 变量\\"}],\\"：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"function MyButton() {\\\\n const [count, setCount] = useState(0);\\\\n // ...\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"你将从 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"useState\\"}],\\" 中获得两样东西：当前的 state（\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\"），以及用于更新它的函数（\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"setCount\\"}],\\"）。你可以给它们起任何名字，但按照惯例会像 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"[something, setSomething]\\"}],\\" 这样为它们命名。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"第一次显示按钮时，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 的值为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"0\\"}],\\"，因为你把 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"0\\"}],\\" 传给了 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"useState()\\"}],\\"。当你想改变 state 时，调用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"setCount()\\"}],\\" 并将新的值传递给它。点击该按钮计数器将递增：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{5}\\",\\"children\\":\\"function MyButton() {\\\\n const [count, setCount] = useState(0);\\\\n\\\\n function handleClick() {\\\\n setCount(count + 1);\\\\n }\\\\n\\\\n return (\\\\n \\u003cbutton onClick={handleClick}\\u003e\\\\n Clicked {count} times\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"React 将再次调用你的组件函数。第一次 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 变成 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"1\\"}],\\"。接着点击会变成 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"2\\"}],\\"。继续点击会逐步递增。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"如果你多次渲染同一个组件，每个组件都会拥有自己的 state。你可以尝试点击不同的按钮：\\"}],\\"\\\\n\\"]}],[\\"$r\\",\\"Sandpack\\",null,{\\"children\\":[[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"import { useState } from 'react';\\\\n\\\\nexport default function MyApp() {\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eCounters that update separately\\u003c/h1\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\\\nfunction MyButton() {\\\\n const [count, setCount] = useState(0);\\\\n\\\\n function handleClick() {\\\\n setCount(count + 1);\\\\n }\\\\n\\\\n return (\\\\n \\u003cbutton onClick={handleClick}\\u003e\\\\n Clicked {count} times\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-css\\",\\"children\\":\\"button {\\\\n display: block;\\\\n margin-bottom: 5px;\\\\n}\\\\n\\"}]}]]}],[\\"$r\\",\\"MaxWidth\\",\\"166\\",{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"注意，每个按钮会 “记住” 自己的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\"，而不影响其他按钮。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"using-hooks\\",\\"children\\":\\"使用 Hook \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"以 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"use\\"}],\\" 开头的函数被称为 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"Hook\\"}],\\"。\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"useState\\"}],\\" 是 React 提供的一个内置 Hook。你可以在 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"/reference/react\\",\\"children\\":\\"React API 参考\\"}],\\" 中找到其他内置的 Hook。你也可以通过组合现有的 Hook 来编写属于你自己的 Hook。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"Hook 比普通函数更为严格。你只能在你的组件（或其他 Hook）的 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"顶层\\"}],\\" 调用 Hook。如果你想在一个条件或循环中使用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"useState\\"}],\\"，请提取一个新的组件并在组件内部使用它。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"sharing-data-between-components\\",\\"children\\":\\"组件间共享数据 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"在前面的示例中，每个 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 都有自己独立的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\"，当每个按钮被点击时，只有被点击按钮的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 才会发生改变：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"DiagramGroup\\",null,{\\"children\\":[[\\"$r\\",\\"Diagram\\",null,{\\"name\\":\\"sharing\_data\_child\\",\\"height\\":367,\\"width\\":407,\\"alt\\":\\"Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. Both MyButton components contain a count with value zero.\\",\\"children\\":[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"起初，每个 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" state 均为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"0\\"}]]}]}],[\\"$r\\",\\"Diagram\\",null,{\\"name\\":\\"sharing\_data\_child\_clicked\\",\\"height\\":367,\\"width\\":407,\\"alt\\":\\"The same diagram as the previous, with the count of the first child MyButton component highlighted indicating a click with the count value incremented to one. The second MyButton component still contains value zero.\\",\\"children\\":[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"第一个 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 会将 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 更新为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"1\\"}]]}]}]]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"然而，你经常需要组件 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"共享数据并一起更新\\"}],\\"。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"为了使得 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 组件显示相同的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 并一起更新，你需要将各个按钮的 state “向上” 移动到最接近包含所有按钮的组件之中。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"在这个示例中，它是 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\"：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"DiagramGroup\\",null,{\\"children\\":[[\\"$r\\",\\"Diagram\\",null,{\\"name\\":\\"sharing\_data\_parent\\",\\"height\\":385,\\"width\\":410,\\"alt\\":\\"Diagram showing a tree of three components, one parent labeled MyApp and two children labeled MyButton. MyApp contains a count value of zero which is passed down to both of the MyButton components, which also show value zero.\\",\\"children\\":[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"起初，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" state 为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"0\\"}],\\" 并传递给了两个子组件\\"]}]}],[\\"$r\\",\\"Diagram\\",null,{\\"name\\":\\"sharing\_data\_parent\_clicked\\",\\"height\\":385,\\"width\\":410,\\"alt\\":\\"The same diagram as the previous, with the count of the parent MyApp component highlighted indicating a click with the value incremented to one. The flow to both of the children MyButton components is also highlighted, and the count value in each child is set to one indicating the value was passed down.\\",\\"children\\":[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"点击后，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 将 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" state 更新为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"1\\"}],\\"，并将其传递给两个子组件\\"]}]}]]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"此刻，当你点击任何一个按钮时，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 中的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 都将改变，同时会改变 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 中的两个 count。具体代码如下：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"首先，将 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 的 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"state 上移到\\"}],\\" \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 中：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{2-6,18}\\",\\"children\\":\\"export default function MyApp() {\\\\n const [count, setCount] = useState(0);\\\\n\\\\n function handleClick() {\\\\n setCount(count + 1);\\\\n }\\\\n\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eCounters that update separately\\u003c/h1\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003cMyButton /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\\\nfunction MyButton() {\\\\n // ... we're moving code from here ...\\\\n}\\\\n\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"接着，将 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 中的点击事件处理函数以及 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"state 一同向下传递到\\"}],\\" 每个 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 中。你可以使用 JSX 的大括号向 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 传递信息。就像之前向 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"\\u003cimg\\u003e\\"}],\\" 等内置标签所做的那样:\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{11-12}\\",\\"children\\":\\"export default function MyApp() {\\\\n const [count, setCount] = useState(0);\\\\n\\\\n function handleClick() {\\\\n setCount(count + 1);\\\\n }\\\\n\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eCounters that update together\\u003c/h1\\u003e\\\\n \\u003cMyButton count={count} onClick={handleClick} /\\u003e\\\\n \\u003cMyButton count={count} onClick={handleClick} /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"使用这种方式传递的信息被称作 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"prop\\"}],\\"。此时 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 组件包含了 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" state 以及 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"handleClick\\"}],\\" 事件处理函数，并将它们作为 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"prop 传递给\\"}],\\" 了每个按钮。\\"]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"最后，改变 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyButton\\"}],\\" 以 \\",[\\"$r\\",\\"strong\\",null,{\\"children\\":\\"读取\\"}],\\" 从父组件传递来的 prop：\\"]}],\\"\\\\n\\",[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"meta\\":\\"{1,3}\\",\\"children\\":\\"function MyButton({ count, onClick }) {\\\\n return (\\\\n \\u003cbutton onClick={onClick}\\u003e\\\\n Clicked {count} times\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"当你点击按钮时，\\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"onClick\\"}],\\" 处理程序会启动。每个按钮的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"onClick\\"}],\\" prop 会被设置为 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"MyApp\\"}],\\" 内的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"handleClick\\"}],\\" 函数，所以函数内的代码会被执行。该代码会调用 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"setCount(count + 1)\\"}],\\"，使得 state 变量 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 递增。新的 \\",[\\"$r\\",\\"code\\",null,{\\"children\\":\\"count\\"}],\\" 值会被作为 prop 传递给每个按钮，因此它们每次展示的都是最新的值。这被称为“状态提升”。通过向上移动 state，我们实现了在组件间共享它。\\"]}],\\"\\\\n\\"]}],[\\"$r\\",\\"Sandpack\\",null,{\\"children\\":[[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-js\\",\\"children\\":\\"import { useState } from 'react';\\\\n\\\\nexport default function MyApp() {\\\\n const [count, setCount] = useState(0);\\\\n\\\\n function handleClick() {\\\\n setCount(count + 1);\\\\n }\\\\n\\\\n return (\\\\n \\u003cdiv\\u003e\\\\n \\u003ch1\\u003eCounters that update together\\u003c/h1\\u003e\\\\n \\u003cMyButton count={count} onClick={handleClick} /\\u003e\\\\n \\u003cMyButton count={count} onClick={handleClick} /\\u003e\\\\n \\u003c/div\\u003e\\\\n );\\\\n}\\\\n\\\\nfunction MyButton({ count, onClick }) {\\\\n return (\\\\n \\u003cbutton onClick={onClick}\\u003e\\\\n Clicked {count} times\\\\n \\u003c/button\\u003e\\\\n );\\\\n}\\\\n\\"}]}],[\\"$r\\",\\"pre\\",null,{\\"children\\":[\\"$r\\",\\"code\\",null,{\\"className\\":\\"language-css\\",\\"children\\":\\"button {\\\\n display: block;\\\\n margin-bottom: 5px;\\\\n}\\\\n\\"}]}]]}],[\\"$r\\",\\"MaxWidth\\",\\"last\\",{\\"children\\":[\\"\\\\n\\",[\\"$r\\",\\"h2\\",null,{\\"id\\":\\"next-steps\\",\\"children\\":\\"下一节 \\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":\\"至此，你已经了解了如何编写 React 代码的基本知识。\\"}],\\"\\\\n\\",[\\"$r\\",\\"p\\",null,{\\"children\\":[\\"接下来你可以查看 \\",[\\"$r\\",\\"a\\",null,{\\"href\\":\\"/learn/tutorial-tic-tac-toe\\",\\"children\\":\\"实战教程\\"}],\\" 并把它们付诸实践，用 React 建立第一个迷你应用程序。\\"]}]]}]]","meta":{"title":"快速入门","translators":["x-cold","sooia","KnowsCount","QC-L","Zhou Chenyang"]},"languages":null},"\_\_N\_SSG":true},"page":"/[[...markdownPath]]","query":{"markdownPath":["learn"]},"buildId":"ptwQzutO8VageFnySCpGM","isFallback":false,"gsp":true,"scriptLoader":[]}