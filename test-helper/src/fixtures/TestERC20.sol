//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.1;

contract ERC20 {
//    using SafeMath for uint;

    string internal _name;
    string internal _symbol;
    uint8 internal _decimals;
    uint256 internal _totalSupply;

    mapping (address => uint256) internal _balances;
    mapping (address => mapping (address => uint256)) internal _allowed;

    event Mint(address indexed minter, address indexed account, uint256 amount);
    event Burn(address indexed burner, address indexed account, uint256 amount);

    constructor (string memory nameC, string memory symbolC, uint8 decimalsC, uint256 totalSupplyC) {
        _symbol = symbolC;
        _name = nameC;
        _decimals = decimalsC;
        _totalSupply = totalSupplyC;
        _balances[msg.sender] = totalSupplyC;

        // Ownable
        _owner = msg.sender;
    }

    function name() public view returns (string memory) {
        return _name;
    }

    function symbol() public view returns (string memory) {
        return _symbol;
    }

    function decimals() public view returns (uint8) {
        return _decimals;
    }

    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function transfer(address _to, uint256 _value) public whenNotPaused returns (bool) {
        require(_to != address(0), 'ERC20: to address is not valid');
        require(_value <= _balances[msg.sender], 'ERC20: insufficient balance');

        _balances[msg.sender] = sub(_balances[msg.sender], _value);
        _balances[_to] = add(_balances[_to], _value);

        emit Transfer(msg.sender, _to, _value);

        return true;
    }

    function balanceOf(address _ownerP) public view returns (uint256 balance) {
        return _balances[_ownerP];
    }

    function approve(address _spender, uint256 _value) public whenNotPaused returns (bool) {
        _allowed[msg.sender][_spender] = _value;

        emit Approval(msg.sender, _spender, _value);

        return true;
    }

    function transferFrom(address _from, address _to, uint256 _value) public whenNotPaused returns (bool) {
        require(_from != address(0), 'ERC20: from address is not valid');
        require(_to != address(0), 'ERC20: to address is not valid');
        require(_value <= _balances[_from], 'ERC20: insufficient balance');
        require(_value <= _allowed[_from][msg.sender], 'ERC20: from not allowed');

        _balances[_from] = sub(_balances[_from], _value);
        _balances[_to] = add(_balances[_to], _value);
        _allowed[_from][msg.sender] = sub(_allowed[_from][msg.sender], _value);

        emit Transfer(_from, _to, _value);

        return true;
    }

    function allowance(address _ownerP, address _spender) public view whenNotPaused returns (uint256) {
        return _allowed[_ownerP][_spender];
    }

    function increaseApproval(address _spender, uint _addedValue) public whenNotPaused returns (bool) {
        _allowed[msg.sender][_spender] = add(_allowed[msg.sender][_spender], _addedValue);

        emit Approval(msg.sender, _spender, _allowed[msg.sender][_spender]);

        return true;
    }

    function decreaseApproval(address _spender, uint _subtractedValue) public whenNotPaused returns (bool) {
        uint oldValue = _allowed[msg.sender][_spender];

        if (_subtractedValue > oldValue) {
            _allowed[msg.sender][_spender] = 0;
        } else {
            _allowed[msg.sender][_spender] = sub(oldValue, _subtractedValue);
        }

        emit Approval(msg.sender, _spender, _allowed[msg.sender][_spender]);

        return true;
    }

    function mintTo(address _to, uint _amount) public whenNotPaused onlyOwner {
        require(_to != address(0), 'ERC20: to address is not valid');
        require(_amount > 0, 'ERC20: amount is not valid');


        _totalSupply = add(_totalSupply, _amount);
        _balances[_to] = add(_balances[_to], _amount);

        emit Mint(msg.sender, _to, _amount);
    }

    function burnFrom(address _from, uint _amount) public whenNotPaused onlyOwner {
        require(_from != address(0), 'ERC20: from address is not valid');
        require(_balances[_from] >= _amount, 'ERC20: insufficient balance');

        _balances[_from] = sub(_balances[_from], _amount);
        _totalSupply = sub( _totalSupply, _amount);

        emit Burn(msg.sender, _from, _amount);
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    // SafeMath
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) {
            return 0;
        }

        uint256 c = a * b;
        assert(c / a == b);
        return c;
    }

    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a / b;
        return c;
    }

    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        assert(b <= a);
        return a - b;
    }

    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        assert(c >= a);
        return c;
    }

    // OWnable

    address private _owner;

    modifier onlyOwner() {
        require(isOwner(), "Ownable: caller is not the owner");
        _;
    }

    function owner() public view returns (address) {
        return _owner;
    }

    function isOwner() public view returns (bool) {
        return msg.sender == _owner;
    }

    // Pausable
    event Paused(address account);
    event Unpaused(address account);

    bool private _paused;

    modifier whenNotPaused() {
        require(!_paused, "Pausable: paused");
        _;
    }

    modifier whenPaused() {
        require(_paused, "Pausable: not paused");
        _;
    }

    function paused() public view returns (bool)
    {
        return _paused;
    }

    function pause() public onlyOwner whenNotPaused {
        _paused = true;
        emit Paused(msg.sender);
    }

    function unpause() public onlyOwner whenPaused {
        _paused = false;
        emit Unpaused(msg.sender);
    }
}
