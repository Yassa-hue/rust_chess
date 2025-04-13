use crate::pieces::{Color, MoveOffsets, MovementDirection, Position};


    #[test]
    fn test_color_next() {
        let white = Color::White;
        let black = Color::Black;
        
        assert_eq!(white.next(), Color::Black);
        assert_eq!(black.next(), Color::White);
    }

    #[test]
    fn test_position_new() {
        let pos = Position::new(3, 4);
        assert_eq!(pos.x(), 3);
        assert_eq!(pos.y(), 4);
    }

    #[test]
    fn test_position_from_str() {
        let pos = Position::from_str("3A");
        assert_eq!(pos.x(), 2);  // 3 - 1 = 2
        assert_eq!(pos.y(), 0);  // A - 1 = 0
    }

    #[test]
    fn test_position_calculate_movement_direction_vertical() {
        let pos1 = Position::new(3, 2);
        let pos2 = Position::new(3, 5);

        let direction = pos1.calculate_movement_direction(&pos2);
        assert_eq!(direction.dx(), 0);
        assert_eq!(direction.dy(), 1);  // Moving downward
    }

    #[test]
    fn test_position_calculate_movement_direction_horizontal() {
        let pos1 = Position::new(2, 3);
        let pos2 = Position::new(5, 3);

        let direction = pos1.calculate_movement_direction(&pos2);
        assert_eq!(direction.dx(), 1);  // Moving right
        assert_eq!(direction.dy(), 0);
    }

    #[test]
    fn test_position_calculate_movement_direction_diagonal() {
        let pos1 = Position::new(2, 3);
        let pos2 = Position::new(5, 6);

        let direction = pos1.calculate_movement_direction(&pos2);
        assert_eq!(direction.dx(), 1);  // Moving right
        assert_eq!(direction.dy(), 1);  // Moving down
    }

    #[test]
    fn test_position_add() {
        let pos = Position::new(3, 4);

        // Test adding valid offset
        let new_pos = pos + (2, 2);
        assert!(new_pos.is_some());
        let new_pos = new_pos.unwrap();
        assert_eq!(new_pos.x(), 5);
        assert_eq!(new_pos.y(), 6);

        // Test adding invalid offset (out of bounds)
        let new_pos = pos + (10, 10);
        assert!(new_pos.is_none());
    }

    #[test]
    fn test_position_add_movement_direction() {
        let pos = Position::new(2, 2);
        let direction = MovementDirection::new(1, 1);

        // Test adding valid direction
        let new_pos = pos + direction;
        assert!(new_pos.is_some());
        let new_pos = new_pos.unwrap();
        assert_eq!(new_pos.x(), 3);
        assert_eq!(new_pos.y(), 3);

        // Test adding direction that goes out of bounds
        let direction_out_of_bounds = MovementDirection::new(10, 10);
        let new_pos = pos + direction_out_of_bounds;
        assert!(new_pos.is_none());
    }

    #[test]
    fn test_move_offsets_apply_offsets_once() {
        let position = Position::new(3, 3);
        let offsets = vec![(1, 0), (0, 1)];
        let move_offsets = MoveOffsets::new_appliable_once(offsets);

        let result = move_offsets.apply_offsets(position);
        let expected_positions = vec![
            Position::new(4, 3),  // Right
            Position::new(3, 4),  // Up
        ];
        assert_eq!(result.len(), 2);
        assert!(result.contains(&expected_positions[0]));
        assert!(result.contains(&expected_positions[1]));
    }

    #[test]
    fn test_move_offsets_apply_offsets_multiple() {
        let position = Position::new(3, 3);
        let offsets = vec![(1, 0), (0, 1)];
        let move_offsets = MoveOffsets::new_appliable_multiple(offsets);

        let result = move_offsets.apply_offsets(position);
        let expected_positions = vec![
            Position::new(4, 3),  // Right
            Position::new(3, 4),  // Up
            Position::new(5, 3),  // Right
            Position::new(3, 5),  // Up
        ];
        assert_eq!(result.len(), 4);
        for expected in expected_positions {
            assert!(result.contains(&expected));
        }
    }
