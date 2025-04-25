-- Synthetic data generation for Onyx Horizons database

-- Clients
INSERT INTO client (name, inn, address, contact_person_email, contact_person_name, is_vip) VALUES
('City of Newbrook', '1234567890', '100 City Hall Plaza, Newbrook', 'mayor@newbrook.gov', 'Mayor Johnson', TRUE),
('Quantum Properties', '9876543210', '500 Business Avenue, Newbrook', 'ceo@quantum.com', 'Sarah Williams', TRUE),
('GreenSpace Development', '2468013579', '123 Park Road, Greenfield', 'contact@greenspace.org', 'Michael Green', FALSE),
('Bridge County Administration', '1357924680', '200 County Road, Bridge County', 'admin@bridgecounty.gov', 'Thomas Bridge', FALSE),
('PowerGrid Solutions', '5647382910', '800 Energy Way, Powertown', 'operations@powergrid.com', 'Elena Rodriguez', TRUE),
('Urban Housing Authority', '6758493021', '300 Housing Street, Newbrook', 'director@urbanhousing.org', 'Daniel Kim', FALSE),
('Highway Development Corp', '3948576201', '450 Transport Lane, Roadside', 'projects@highway.com', 'Robert Lanes', TRUE),
('Riverside Parks Trust', '8172635409', '75 River Road, Riverside', 'trust@riversideparks.org', 'Emma Waters', FALSE),
('Mountain View Estates', '2039485761', '600 Mountain Road, Highland', 'estates@mountainview.com', 'James Heights', FALSE),
('Solar Future Energy', '9273645810', '900 Sunshine Boulevard, Solarville', 'future@solarenergy.com', 'Sophia Ray', TRUE);

-- Employees - Technical Personnel
INSERT INTO employee (class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
-- Engineers (senior positions)
('technical_personnel', 'Alexander', 'Smith', 'James', 'male', 'photos/asmith.jpg', 95000, '+1-555-123-4567'),
('technical_personnel', 'Victoria', 'Johnson', 'Elizabeth', 'female', 'photos/vjohnson.jpg', 92000, '+1-555-234-5678'),
('technical_personnel', 'William', 'Brown', 'Robert', 'male', 'photos/wbrown.jpg', 94000, '+1-555-345-6789'),
-- Technologists (mid positions)
('technical_personnel', 'Emma', 'Davis', 'Marie', 'female', 'photos/edavis.jpg', 75000, '+1-555-456-7890'),
('technical_personnel', 'Michael', 'Wilson', 'Thomas', 'male', 'photos/mwilson.jpg', 72000, '+1-555-567-8901'),
('technical_personnel', 'Olivia', 'Miller', 'Grace', 'female', 'photos/omiller.jpg', 73000, '+1-555-678-9012'),
-- Technicians (regular positions)
('technical_personnel', 'James', 'Taylor', 'Edward', 'male', 'photos/jtaylor.jpg', 65000, '+1-555-789-0123'),
('technical_personnel', 'Sophia', 'Anderson', 'Claire', 'female', 'photos/sanderson.jpg', 62000, '+1-555-890-1234'),
('technical_personnel', 'Benjamin', 'Thomas', 'David', 'male', 'photos/bthomas.jpg', 63000, '+1-555-901-2345'),
('technical_personnel', 'Isabella', 'White', 'Jane', 'female', null, 64000, '+1-555-012-3456');

-- Technical Personnel
INSERT INTO technical_personnel (id, qualification, position, education_level, software_skills, is_project_manager) VALUES
(1, 'engineer', 'foreman', 'PhD Civil Engineering', ARRAY['AutoCAD', 'Revit', 'Primavera P6'], TRUE),
(2, 'engineer', 'master', 'MSc Structural Engineering', ARRAY['AutoCAD', 'SAP2000', 'ETABS'], TRUE),
(3, 'engineer', 'foreman', 'MSc Construction Management', ARRAY['MS Project', 'BIM 360', 'Procore'], TRUE),
(4, 'technologist', 'master', 'BSc Building Technology', ARRAY['Revit', 'Navisworks', 'Bluebeam'], FALSE),
(5, 'technologist', 'foreman', 'BSc Construction Technology', ARRAY['SketchUp', 'AutoCAD', 'PlanGrid'], FALSE),
(6, 'technologist', null, 'BSc Civil Technology', ARRAY['AutoCAD', 'Civil 3D', 'Tekla'], TRUE),
(7, 'technician', null, 'Associate Degree Construction', ARRAY['AutoCAD', 'Bluebeam'], FALSE),
(8, 'technician', null, 'Technical Diploma', ARRAY['QCAD', 'SolidWorks'], FALSE),
(9, 'technician', null, 'Construction Certificate', ARRAY['AutoCAD LT', 'PlanGrid'], FALSE),
(10, 'technician', null, 'Technical Diploma', ARRAY['AutoCAD', 'Revit LT'], FALSE);

-- Engineers
INSERT INTO engineer (id, pe_license_id) VALUES
(1, 123456),
(2, 234567),
(3, 345678);

-- Technologists
INSERT INTO technologist (id, management_tools) VALUES
(4, ARRAY['Asana', 'JIRA', 'Trello']),
(5, ARRAY['Procore', 'Monday.com', 'SmartSheet']),
(6, ARRAY['MS Project', 'Primavera', 'Fieldwire']);

-- Technicians
INSERT INTO technician (id, safety_training_level) VALUES
(7, 'Level 3'),
(8, 'Level 2'),
(9, 'Level 2'),
(10, 'Level 1');

-- Workers
INSERT INTO employee (class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
-- Electricians
('worker', 'John', 'Martinez', 'Luis', 'male', 'photos/jmartinez.jpg', 58000, '+1-555-111-2222'),
('worker', 'Emily', 'Rodriguez', 'Sofia', 'female', 'photos/erodriguez.jpg', 56000, '+1-555-222-3333'),
-- Plumbers
('worker', 'David', 'Garcia', 'Miguel', 'male', 'photos/dgarcia.jpg', 55000, '+1-555-333-4444'),
('worker', 'Sarah', 'Lopez', 'Anna', 'female', null, 54000, '+1-555-444-5555'),
-- Welders
('worker', 'Robert', 'Gonzalez', 'Carlos', 'male', 'photos/rgonzalez.jpg', 59000, '+1-555-555-6666'),
('worker', 'Jessica', 'Perez', 'Maria', 'female', 'photos/jperez.jpg', 57000, '+1-555-666-7777'),
-- Drivers
('worker', 'Charles', 'Sanchez', 'Jose', 'male', null, 52000, '+1-555-777-8888'),
('worker', 'Amanda', 'Rivera', 'Elena', 'female', 'photos/arivera.jpg', 51000, '+1-555-888-9999'),
-- Masons
('worker', 'Thomas', 'Torres', 'Alejandro', 'male', 'photos/ttorres.jpg', 53000, '+1-555-999-0000'),
('worker', 'Lisa', 'Flores', 'Carmen', 'female', 'photos/lflores.jpg', 52500, '+1-555-000-1111'),
-- More workers of various types to fill brigades
('worker', 'Paul', 'Nguyen', 'Van', 'male', null, 54500, '+1-555-112-2233'),
('worker', 'Jennifer', 'Kim', 'Min', 'female', 'photos/jkim.jpg', 53000, '+1-555-223-3344'),
('worker', 'Mark', 'Singh', 'Raj', 'male', 'photos/msingh.jpg', 56500, '+1-555-334-4455'),
('worker', 'Laura', 'Patel', 'Priya', 'female', null, 55500, '+1-555-445-5566'),
('worker', 'Kevin', 'Wong', 'Lee', 'male', 'photos/kwong.jpg', 57500, '+1-555-556-6677'),
('worker', 'Natalie', 'Chen', 'Lin', 'female', 'photos/nchen.jpg', 52000, '+1-555-667-7788'),
('worker', 'Brian', 'Shah', 'Kumar', 'male', null, 54000, '+1-555-778-8899'),
('worker', 'Rachel', 'Ali', 'Fatima', 'female', 'photos/rali.jpg', 53500, '+1-555-889-9900'),
('worker', 'Eric', 'Kumar', 'Vikram', 'male', 'photos/ekumar.jpg', 58500, '+1-555-990-0011'),
('worker', 'Katherine', 'Park', 'Min', 'female', null, 56000, '+1-555-001-1122');

-- Worker professions
INSERT INTO worker (id, profession, union_name) VALUES
(11, 'electrician', 'Electrical Workers Union'),
(12, 'electrician', 'Electrical Workers Union'),
(13, 'plumber', 'Plumbers Association'),
(14, 'plumber', 'Plumbers Association'),
(15, 'welder', 'Metal Workers Union'),
(16, 'welder', 'Metal Workers Union'),
(17, 'driver', 'Teamsters'),
(18, 'driver', 'Teamsters'),
(19, 'mason', 'Bricklayers Union'),
(20, 'mason', 'Bricklayers Union'),
(21, 'electrician', 'Electrical Workers Union'),
(22, 'plumber', null),
(23, 'welder', 'Metal Workers Union'),
(24, 'driver', 'Teamsters'),
(25, 'mason', 'Bricklayers Union'),
(26, 'electrician', null),
(27, 'plumber', 'Plumbers Association'),
(28, 'welder', null),
(29, 'driver', 'Teamsters'),
(30, 'mason', 'Bricklayers Union');

-- Worker specialties
INSERT INTO electrirican (id, voltage_specializaition) VALUES
(11, 'High Voltage'),
(12, 'Low Voltage/Residential'),
(21, 'Industrial'),
(26, 'Commercial');

INSERT INTO plumber (id, pipe_specialization) VALUES
(13, 'Commercial Systems'),
(14, 'Residential'),
(22, 'Industrial/High Pressure'),
(27, 'Gas Lines');

INSERT INTO welder (id, welding_machine) VALUES
(15, 'MIG Welder'),
(16, 'TIG Welder'),
(23, 'Arc Welder'),
(28, 'Spot Welder');

INSERT INTO driver (id, vehicle_type, number_of_accidents) VALUES
(17, 'Dump Truck', 0),
(18, 'Cement Mixer', 1),
(24, 'Flatbed Truck', 0),
(29, 'Heavy Equipment Transporter', 2);

INSERT INTO mason (id, hq_restoration_skills) VALUES
(19, TRUE),
(20, FALSE),
(25, TRUE),
(30, FALSE);

-- Departments
INSERT INTO department (supervisor_id, name) VALUES
(1, 'Commercial Construction'),
(2, 'Infrastructure'),
(3, 'Residential Construction');

-- Areas
INSERT INTO area (department_id, supervisor_id, name) VALUES
(1, 4, 'Power Plants'),
(1, 5, 'Commercial Buildings'),
(2, 3, 'Roads and Highways'),
(2, 6, 'Bridges'),
(3, 1, 'Housing Developments'),
(3, 2, 'Parks and Recreation');

-- Brigades
INSERT INTO brigade (brigadier_id) VALUES
(11), -- Electrician brigade
(13), -- Plumber brigade
(15), -- Welder brigade
(17), -- Driver brigade
(19); -- Mason brigade

-- Worker assignments to brigades
INSERT INTO assignment (brigade_id, worker_id) VALUES
-- Electrician Brigade
(1, 11), -- Brigadier
(1, 12),
(1, 21),
(1, 26),
-- Plumber Brigade
(2, 13), -- Brigadier
(2, 14),
(2, 22),
(2, 27),
-- Welder Brigade
(3, 15), -- Brigadier
(3, 16),
(3, 23),
(3, 28),
-- Driver Brigade
(4, 17), -- Brigadier
(4, 18),
(4, 24),
(4, 29),
-- Mason Brigade
(5, 19), -- Brigadier
(5, 20),
(5, 25),
(5, 30);

-- Sites
INSERT INTO site (area_id, client_id, name, type, location, risk_level, description) VALUES
-- Power Plants
(1, 5, 'Newbrook Solar Farm', 'power_plant', point(40.7128, -74.0060), 'high', 'Solar power plant with 500 acres of solar panels'),
(1, 10, 'Midtown Energy Center', 'power_plant', point(41.8781, -87.6298), 'high', 'Natural gas power plant with 600MW capacity'),
-- Roads
(3, 7, 'West Highway Extension', 'road', point(37.7749, -122.4194), 'medium', 'Highway expansion project with 6 lanes'),
(3, 1, 'Downtown Connector', 'road', point(39.9526, -75.1652), 'low', 'City road reconstruction with bike lanes'),
-- Housing
(5, 2, 'Sunset Towers', 'housing', point(34.0522, -118.2437), 'medium', 'Luxury apartment complex with 200 units'),
(5, 6, 'Meadow View Residences', 'housing', point(33.4484, -112.0740), 'low', 'Affordable housing development with 150 units'),
-- Bridges
(4, 4, 'Harbor Crossing Bridge', 'bridge', point(37.8199, -122.4783), 'high', 'Major suspension bridge spanning 2000m'),
(4, 1, 'Riverside Pedestrian Bridge', 'bridge', point(38.6270, -90.1994), 'medium', 'Pedestrian bridge connecting city parks'),
-- Parks
(6, 8, 'Central Recreation Park', 'park', point(40.7829, -73.9654), 'low', 'Urban park with playground and gardens'),
(6, 3, 'Waterfront Park', 'park', point(47.6062, -122.3321), 'low', 'Waterfront park with recreational facilities');

-- Site specific details
INSERT INTO power_plant (site_id, energy_output, energy_source, is_grid_connected) VALUES
(1, 75.5, 'Solar', TRUE),
(2, 600.0, 'Natural Gas', TRUE);

INSERT INTO road (site_id, length, lanes, surface) VALUES
(3, 25000.0, 6, 'Asphalt'),
(4, 5000.0, 4, 'Concrete');

INSERT INTO housing (site_id, number_of_floors, number_of_entrances, type, energy_efficiency) VALUES
(5, 20, 4, 'Luxury Apartment', 'high'),
(6, 10, 8, 'Affordable Housing', 'medium');

INSERT INTO bridge (site_id, length, road_material, max_load) VALUES
(7, 2000.0, 'Asphalt Concrete', 50000.0),
(8, 500.0, 'Composite Decking', 10000.0);

INSERT INTO park (site_id, area, has_playground, has_lighting) VALUES
(9, 5.5, TRUE, TRUE),
(10, 12.2, TRUE, FALSE);

-- Equipment
INSERT INTO equipment (name, amount, purchase_date, purchase_cost, fuel_type) VALUES
('Excavator CAT 320', 10, '2020-05-15', 250000.00, 'Diesel'),
('Crane Liebherr LTM 1200', 5, '2019-08-20', 800000.00, 'Diesel'),
('Concrete Mixer Truck', 15, '2021-01-10', 150000.00, 'Diesel'),
('Bulldozer', 8, '2020-03-25', 350000.00, 'Diesel'),
('Generator', 20, '2022-02-10', 25000.00, 'Gasoline'),
('Welding Machine', 30, '2021-11-05', 8000.00, 'Electric'),
('Scaffolding Set', 50, '2022-01-15', 5000.00, NULL),
('Water Pump', 25, '2021-06-20', 12000.00, 'Electric'),
('Air Compressor', 15, '2020-09-12', 15000.00, 'Electric'),
('Forklift', 12, '2019-12-05', 65000.00, 'Propane');

-- Equipment Allocation
INSERT INTO equipment_allocation (equipment_id, department_id, site_id, amount, period_start, period_end) VALUES
-- Allocated to sites
(1, 1, 1, 1, '2022-01-01', '2023-05-30'),
(2, 1, 1, 1, '2022-01-01', '2023-05-30'),
(3, 1, 1, 2, '2022-01-01', '2023-05-30'),
(4, 1, 1, 1, '2022-01-01', '2023-05-30'),
(5, 1, 1, 2, '2022-01-01', '2023-05-30'),
(3, 2, 3, 3, '2022-02-15', '2023-08-15'),
(4, 2, 3, 2, '2022-02-15', '2023-08-15'),
(1, 2, 3, 2, '2022-02-15', '2023-08-15'),
(2, 1, 5, 1, '2022-03-10', '2023-10-30'),
(3, 1, 5, 2, '2022-03-10', '2023-10-30'),
(6, 1, 5, 5, '2022-03-10', '2023-10-30'),
(7, 1, 5, 10, '2022-03-10', '2023-10-30'),
(1, 2, 7, 1, '2022-04-01', '2023-12-15'),
(2, 2, 7, 2, '2022-04-01', '2023-12-15'),
(4, 2, 7, 1, '2022-04-01', '2023-12-15'),
(6, 2, 7, 4, '2022-04-01', '2023-12-15'),
(7, 2, 7, 15, '2022-04-01', '2023-12-15'),
(3, 3, 9, 1, '2022-05-20', '2022-12-20'),
(4, 3, 9, 1, '2022-05-20', '2022-12-20'),
(8, 3, 9, 3, '2022-05-20', '2022-12-20');

-- Materials
INSERT INTO material (name, cost, units) VALUES
('Concrete', 95.00, 'cubic meter'),
('Steel Rebar', 1100.00, 'ton'),
('Bricks', 0.75, 'piece'),
('Lumber', 650.00, 'cubic meter'),
('Glass', 55.00, 'square meter'),
('Copper Wire', 8.50, 'meter'),
('PVC Pipe', 12.00, 'meter'),
('Insulation', 18.00, 'square meter'),
('Asphalt', 85.00, 'ton'),
('Paint', 45.00, 'gallon'),
('Roofing Shingles', 32.00, 'bundle'),
('Nails', 3.50, 'pound'),
('Sand', 35.00, 'ton'),
('Gravel', 40.00, 'ton'),
('Solar Panel', 300.00, 'piece');

-- Tasks
INSERT INTO task (site_id, brigade_id, period_start, expected_period_end, actual_period_end, name, description) VALUES
-- Power Plant 1
(1, 3, '2022-01-15', '2022-03-15', '2022-03-20', 'Steel Framework', 'Construction of the main steel framework for solar panel mounts'),
(1, 1, '2022-03-25', '2022-05-25', '2022-05-20', 'Electrical Systems', 'Installation of all electrical systems and connections'),
(1, 5, '2022-01-10', '2022-02-28', '2022-03-10', 'Foundation Construction', 'Pouring concrete foundations for support structures'),
-- Road 3
(3, 4, '2022-02-20', '2022-05-20', '2022-06-15', 'Grading and Excavation', 'Preparing the roadbed for construction'),
(3, 5, '2022-06-20', '2022-08-20', '2022-08-25', 'Concrete Base Layer', 'Installing the concrete base for the highway'),
-- Housing 5
(5, 5, '2022-03-15', '2022-06-15', '2022-07-01', 'Foundation and Structure', 'Building the main structure and foundation'),
(5, 2, '2022-07-10', '2022-09-10', '2022-09-15', 'Plumbing Installation', 'Installation of all plumbing systems'),
(5, 1, '2022-09-20', '2022-11-20', '2022-11-15', 'Electrical Systems', 'Installation of all electrical systems'),
-- Bridge 7
(7, 3, '2022-04-10', '2022-08-10', '2022-09-15', 'Steel Support Structure', 'Construction of main steel support structures'),
(7, 5, '2022-09-20', '2023-01-20', '2023-02-10', 'Bridge Deck Construction', 'Construction of the bridge road deck'),
-- Park 9
(9, 5, '2022-05-25', '2022-07-25', '2022-07-20', 'Walkways and Hardscaping', 'Construction of walkways and plaza areas'),
(9, 4, '2022-08-01', '2022-09-15', '2022-09-10', 'Landscaping', 'Planting trees, shrubs, and installing irrigation'),
-- Tasks without actual completion yet
(2, 5, '2022-09-01', '2022-12-01', NULL, 'Foundation Construction', 'Pouring concrete foundations'),
(2, 3, '2022-12-10', '2023-03-10', NULL, 'Steel Framework', 'Construction of the main steel framework'),
(4, 4, '2022-10-15', '2023-01-15', NULL, 'Road Excavation', 'Excavation and preparation of road bed'),
(6, 5, '2022-11-01', '2023-02-01', NULL, 'Foundation Construction', 'Pouring concrete foundations for housing');

-- Expenditures
INSERT INTO expenditure (task_id, material_id, expected_amount, actuial_amount) VALUES
-- Power Plant - Steel Framework
(1, 2, 75.0, 78.5),  -- Steel Rebar
(1, 4, 30.0, 32.0),  -- Lumber
-- Power Plant - Electrical Systems
(2, 6, 5000.0, 4800.0),  -- Copper Wire
(2, 15, 500.0, 500.0),   -- Solar Panels
-- Power Plant - Foundation Construction
(3, 1, 200.0, 210.0),  -- Concrete
(3, 2, 15.0, 16.5),    -- Steel Rebar
(3, 13, 50.0, 48.0),   -- Sand
(3, 14, 60.0, 62.0),   -- Gravel
-- Road - Grading and Excavation
(4, 13, 500.0, 520.0),  -- Sand
(4, 14, 700.0, 750.0),  -- Gravel
-- Road - Concrete Base Layer
(5, 1, 1000.0, 1050.0), -- Concrete
(5, 2, 50.0, 55.0),     -- Steel Rebar
-- Housing - Foundation and Structure
(6, 1, 300.0, 305.0),  -- Concrete
(6, 2, 25.0, 27.0),    -- Steel Rebar
(6, 3, 15000.0, 15500.0), -- Bricks
(6, 4, 50.0, 48.0),    -- Lumber
-- Housing - Plumbing Installation
(7, 7, 2000.0, 1950.0), -- PVC Pipe
-- Housing - Electrical Systems
(8, 6, 3000.0, 2900.0), -- Copper Wire
-- Bridge - Steel Support Structure
(9, 2, 200.0, 215.0),  -- Steel Rebar
-- Bridge - Bridge Deck Construction
(10, 1, 500.0, 530.0), -- Concrete
(10, 9, 100.0, 105.0), -- Asphalt
-- Park - Walkways
(11, 1, 150.0, 145.0), -- Concrete
(11, 13, 30.0, 28.0),  -- Sand
-- Park - Landscaping
(12, 10, 50.0, 45.0),  -- Paint
-- Planned expenditures for incomplete tasks
(13, 1, 400.0, NULL),  -- Concrete
(13, 2, 30.0, NULL),   -- Steel Rebar
(14, 2, 150.0, NULL),  -- Steel Rebar
(15, 13, 300.0, NULL), -- Sand
(15, 14, 450.0, NULL), -- Gravel
(16, 1, 250.0, NULL),  -- Concrete
(16, 3, 20000.0, NULL); -- Bricks
