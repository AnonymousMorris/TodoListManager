This app is operated with purely the keyboard with keybindings heavily borrowed from vim. 
### Modes
Similar to vim, there is a Normal, Insert, and Visual mode. You can move around in normal mode, then enter insert mode when you want to edit a todo item. Visual mode will let you select multiple todo items and let you perform actions on them all at the same time.

| keys | effect |
| --------------- | ---------------- |
| i               | enter insert mode|
| \<esc\>, \<ctrl\> + [ | exit insert mode |
| v               | toggle visual mode |
| Enter           | exit insert mode |

### Movements
| keys | movement |
| --------------- | ---------------- |
| h | left |
| l | right |
| j | down |
| k | up |

### Create and Delete
| key | action |
| --------------- | ---------------- |
| a | create todo |
| d | delete todo |
| n | create todolist |
| shift + d | delete todolist |

### Move todos
| keys | action |
| --------------- | ---------------- |
| shift + j | move todo down |
| shift + k | move todo up |
| shift + h | move todolist left |
| shift + l | move todolist right |

### Command
Press ':' while in normal mode to enter command mode
| keys | action |
| --------------- | ---------------- |
| :w | save |
| :q | quit |
| :wq | write and quit |
| clean | deletes all todos marked as completed |

