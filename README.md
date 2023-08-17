# swc-plugin-react-const-elements

Ported from https://babeljs.io/docs/babel-plugin-transform-react-constant-elements

Transforms

```js
const Hr = () => {
  return <hr className="hr" />;
};

const WithChildren = (props) => {
  return <div className={props.className}>
    <hr />
  </div>;
}
```

Into

```js
let _hr, _hr2;

const Hr = () => {
  return _hr || (_hr = <hr className="hr" />);
};

const WithChildren = (props) => {
  return <div className={props.className}>
    {_hr2 || (_hr2 = <hr />)}
  </div>;
}
```